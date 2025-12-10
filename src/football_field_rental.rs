#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(Debug, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone)]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Cancelled,
}

#[type_abi]
#[derive(Debug, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct Slot<M: ManagedTypeApi> {
    pub start: u64,
    pub end: u64,
    pub payer: ManagedAddress<M>,
    pub amount: BigUint<M>,
    pub confirmed: ReservationStatus,
}

#[multiversx_sc::contract]
pub trait FootballFieldRental {

	// storage mappers

    #[storage_mapper("footballFieldManager")]
    fn football_field_manager(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("footballCourtCost")]
    fn football_court_cost(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("participants")]
    fn participants(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("reservedSlot")]
    fn reserved_slot(&self) -> SingleValueMapper<Slot<Self::Api>>;

    // events

	#[event("slot_reserved")]
    fn slot_reserved_event(
        &self,
        #[indexed] payer: &ManagedAddress,
        data: &(u64, u64, BigUint),
    );

    #[event("slot_confirmed")]
    fn slot_confirmed_event(
        &self,
        #[indexed] payer: &ManagedAddress,
        data: &((),),
    );

    #[event("slot_cancelled")]
    fn slot_cancelled_event(
        &self,
        #[indexed] payer: &ManagedAddress,
        data: &((),),
    );

	#[event("manager_changed")]
	fn manager_changed_event(
    	&self,
    	#[indexed] new_manager: &ManagedAddress,
    	data: &((),),
	);

	#[event("court_paid")]
	fn court_paid_event(
    	&self,
    	#[indexed] manager: &ManagedAddress,
    	data: &(BigUint,),
	);

	#[event("court_cost_updated")]
	fn court_cost_updated_event(
    	&self,
    	#[indexed] owner: &ManagedAddress,
    	data: &(BigUint,),
	);


	// endpoints

	#[endpoint(createFootballSlot)]
#[payable("EGLD")]
fn create_football_slot(
    &self,
    start: u64,
    end: u64,
) -> Slot<Self::Api> {

    let caller = self.blockchain().get_caller();
    let payment = self.call_value().egld().clone_value();

    require!(
        self.reserved_slot().is_empty(),
        "A slot is already reserved"
    );

    let required_cost = self.football_court_cost().get();
    require!(payment == required_cost, "Incorrect deposit amount");

    let new_slot = Slot {
        start,
        end,
        payer: caller.clone(),
        amount: payment.clone(),
        confirmed: ReservationStatus::Pending,
    };

    self.reserved_slot().set(new_slot.clone());
    self.participants().insert(caller.clone());

    self.slot_reserved_event(&caller, &(start, end, payment.clone()));

    new_slot
}

#[endpoint(participateToFootballSlot)]
#[payable("EGLD")]
fn participate_to_football_slot(&self) -> u32 {
    let caller = self.blockchain().get_caller();
    let payment = self.call_value().egld().clone_value();

    require!(!self.reserved_slot().is_empty(), "No active slot to join");

    let required_cost = self.football_court_cost().get();
    require!(payment == required_cost, "Incorrect deposit amount");

    let slot = self.reserved_slot().get();

    require!(slot.confirmed == ReservationStatus::Pending, "Slot already confirmed");
    require!(!self.participants().contains(&caller), "User already joined");

    self.participants().insert(caller.clone());
    self.slot_reserved_event(&caller, &(slot.start, slot.end, payment));

    self.participants().len() as u32
}

	#[endpoint(cancelFootballSlot)]
	fn cancel_football_slot(&self) {
    	let caller = self.blockchain().get_caller();

    	require!(
        	!self.reserved_slot().is_empty(),
        	"No active slot to cancel"
    	);

    	let slot = self.reserved_slot().get();

    	require!(
        	slot.payer == caller,
        	"Only the creator can cancel the slot"
    	);

    	require!(
        	slot.confirmed == ReservationStatus::Pending,
        	"Cannot cancel a confirmed slot"
    	);

    	for participant in self.participants().iter() {
        	self.send().direct_egld(
            	&participant,
            	&slot.amount,
        	);
    	}

    	self.participants().clear();
    	self.reserved_slot().clear();

    	self.slot_cancelled_event(
        	&caller,
        	&((),),
    	);
	}

	#[endpoint(setFootballFieldManager)]
	fn set_football_field_manager(&self, new_manager: ManagedAddress) {
    	let caller = self.blockchain().get_caller();
    	let owner = self.blockchain().get_owner_address();

    	require!(
        	caller == owner,
        	"Only the contract owner may set the field manager"
    	);

    	self.football_field_manager().set(new_manager.clone());

    	self.manager_changed_event(&new_manager, &((),));
	}

#[endpoint(payCourt)]
fn pay_court(&self) -> BigUint<Self::Api> {

    require!(!self.reserved_slot().is_empty(), "No slot exists");

    let slot = self.reserved_slot().get();
    let manager = self.football_field_manager().get();

    require!(manager != ManagedAddress::zero(), "Manager not set");
    require!(slot.confirmed == ReservationStatus::Confirmed, "Not confirmed");

    let participant_count = self.participants().len() as u32;
    let total_payment = slot.amount.clone() * BigUint::from(participant_count);

    self.send().direct_egld(&manager, &total_payment);

    self.participants().clear();
    self.reserved_slot().clear();

    self.court_paid_event(&manager, &(total_payment.clone(),));

    total_payment
}

	#[endpoint(setFootballCourtCost)]
	fn set_football_court_cost(&self, new_cost: BigUint) {
    	let caller = self.blockchain().get_caller();
    	let owner = self.blockchain().get_owner_address();

    	require!(
        	caller == owner,
        	"Only the contract owner may set the court cost"
    	);

    	self.football_court_cost().set(new_cost.clone());

    	self.court_cost_updated_event(
        	&owner,
        	&(new_cost.clone(),),
    	);
	}

	#[endpoint(confirmSlot)]
fn confirm_slot(&self) -> ReservationStatus {
    let caller = self.blockchain().get_caller();

    require!(!self.reserved_slot().is_empty(), "No active slot");
    let mut slot = self.reserved_slot().get();

    require!(slot.confirmed == ReservationStatus::Pending, "Already confirmed");

    let manager = self.football_field_manager().get();
    let owner = self.blockchain().get_owner_address();

    require!(caller == manager || caller == owner, "Not authorized");

    slot.confirmed = ReservationStatus::Confirmed;
    self.reserved_slot().set(slot.clone());

    self.slot_confirmed_event(&caller, &((),));

    slot.confirmed
}

	// view 

	#[view(getSlotStatus)]
	fn get_slot_status(&self)
    	-> (
        	Option<Slot<Self::Api>>,
        	ManagedVec<ManagedAddress<Self::Api>>,
        	BigUint<Self::Api>,
        	ReservationStatus
    	)
	{
    	if self.reserved_slot().is_empty() {
        	return (
            	None,
            	ManagedVec::new(),
            	BigUint::zero(),
        	    ReservationStatus::Cancelled,
    	    );
	    }

	    let slot = self.reserved_slot().get();

    	let mut participants_vec = ManagedVec::new();
    	for p in self.participants().iter() {
    	    participants_vec.push(p);
	    }

    	let participants_count = self.participants().len() as u32;
    	let participants_big = BigUint::from(participants_count);
    	let total_amount = slot.amount.clone() * participants_big;

    	(
        	Some(slot.clone()),
        	participants_vec,
        	total_amount,
        	slot.confirmed.clone(),
    	)
	}

	// init

    #[init]
    fn init(&self, manager: ManagedAddress, court_cost: BigUint) {
        self.football_field_manager().set(manager);
        self.football_court_cost().set(court_cost);
    }
}
