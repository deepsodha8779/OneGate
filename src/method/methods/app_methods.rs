use log::info;
use serde_json::Value;

use super::amenity::AmenityMethods;
use super::complaint::ComplaintMethods;
use super::entry_log::EntryLogMethods;
use super::guest::GuestMethods;
use super::home::HomeMethods;
use super::member::MemberMethods;
use super::notice::NoticeMethods;
use super::notification::NotificationMethods;
use super::owner::OwnerMethods;
use super::payment::PaymentMethods;
use super::security::SecurityMethods;
use super::service_provider::ServiceProviderMethods;
use super::society::SocietyMethods;
use super::staff::StaffMethods;
use super::swimming_pool::SwimmingPoolMethods;
use super::user::UserMethods;
use crate::method::convention::{ErrorData, Request};
use crate::rpc::rpc::Rpc;

#[allow(clippy::large_enum_variant)]
pub enum AppMethods {
    Home(HomeMethods),
    Society(SocietyMethods),
    Amenity(AmenityMethods),
    EntryLog(EntryLogMethods),
    Security(SecurityMethods),
    ServiceProvider(ServiceProviderMethods),
    User(UserMethods),
    Payment(PaymentMethods),
    Notice(NoticeMethods),
    Complaint(ComplaintMethods),
    Guest(GuestMethods),
    Notification(NotificationMethods),
    Owner(OwnerMethods),
    Staff(StaffMethods),
    SwimmingPool(SwimmingPoolMethods),
    Member(MemberMethods),
}

impl Rpc for AppMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        info!(target: "AppMethods::from_name", "str: {:?}, data: {:?}", str, data);
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, elements)) = names.split_first() {
            match *first {
                "Home" => Ok(AppMethods::Home(HomeMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Society" => Ok(AppMethods::Society(SocietyMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Amenity" => Ok(AppMethods::Amenity(AmenityMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "EntryLog" => Ok(AppMethods::EntryLog(EntryLogMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Security" => Ok(AppMethods::Security(SecurityMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "ServiceProvider" => Ok(AppMethods::ServiceProvider(
                    ServiceProviderMethods::from_name(&elements.join("::"), data)?,
                )),
                "User" => Ok(AppMethods::User(UserMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Payment" => Ok(AppMethods::Payment(PaymentMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Notice" => Ok(AppMethods::Notice(NoticeMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Complaint" => Ok(AppMethods::Complaint(ComplaintMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Guest" => Ok(AppMethods::Guest(GuestMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Member" => Ok(AppMethods::Member(MemberMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Notification" => Ok(AppMethods::Notification(NotificationMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Owner" => Ok(AppMethods::Owner(OwnerMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "Staff" => Ok(AppMethods::Staff(StaffMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),
                "SwimmingPool" => Ok(AppMethods::SwimmingPool(SwimmingPoolMethods::from_name(
                    &elements.join("::"),
                    data,
                )?)),

                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };
        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        println!("{}", namespace);
        match self {
            AppMethods::Home(home) => home.to_rpc("Home"),
            AppMethods::Society(society) => society.to_rpc("Society"),
            AppMethods::Amenity(amenity) => amenity.to_rpc("Amenity"),
            AppMethods::EntryLog(entrylog) => entrylog.to_rpc("EntryLog"),
            AppMethods::Security(security) => security.to_rpc("Security"),
            AppMethods::ServiceProvider(service_provider) => {
                service_provider.to_rpc("ServiceProvider")
            }
            AppMethods::User(user) => user.to_rpc("User"),
            AppMethods::Payment(payment) => payment.to_rpc("Payment"),
            AppMethods::Notice(notice) => notice.to_rpc("Notice"),
            AppMethods::Complaint(complaint) => complaint.to_rpc("Notice"),
            AppMethods::Guest(guest) => guest.to_rpc("Guest"),
            AppMethods::Member(guest) => guest.to_rpc("Member"),
            AppMethods::Notification(notification) => notification.to_rpc("Notification"),
            AppMethods::Owner(owner) => owner.to_rpc("Owner"),
            AppMethods::Staff(staff) => staff.to_rpc("Staff"),
            AppMethods::SwimmingPool(swimmingpool) => swimmingpool.to_rpc("SwimmingPool"),
        }
    }
}
