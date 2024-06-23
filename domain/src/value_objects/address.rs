use crate::services::check_address_exists_service::CheckAddressExistsService;
use crate::unvalidated::unvalidated_address::UnvalidatedAddress;
use crate::value_objects::string50::String50;
use std::rc::Rc;

#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Address {
    address_line1: Rc<String50>,
    address_line2: Rc<String50>,
    address_line3: Rc<String50>,
    address_line4: Rc<String50>,
    city: Rc<String50>,
    zip_code: Rc<String50>,
}

impl Address {
    pub async fn create(
        check_address_service: &impl CheckAddressExistsService,
        unvalidated_address: UnvalidatedAddress,
    ) -> Result<Self, &'static str> {
        let checked_address = check_address_service.check(&unvalidated_address).await;

        let address_line1 = String50::create(checked_address.0.address_line1).unwrap();
        let address_line2 = String50::create(checked_address.0.address_line2).unwrap();
        let address_line3 = String50::create(checked_address.0.address_line3).unwrap();
        let address_line4 = String50::create(checked_address.0.address_line4).unwrap();
        let city = String50::create(checked_address.0.city).unwrap();
        let zip_code = String50::create(checked_address.0.zip_code).unwrap();

        let result = Self {
            address_line1: Rc::new(address_line1),
            address_line2: Rc::new(address_line2),
            address_line3: Rc::new(address_line3),
            address_line4: Rc::new(address_line4),
            city: Rc::new(city),
            zip_code: Rc::new(zip_code),
        };

        Ok(result)
    }
}

#[readonly::make]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct CheckedAddress(pub UnvalidatedAddress);
