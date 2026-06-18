use uuid::Uuid;

pub struct CreateOrderCommand {
    pub customer_name: String,
}

pub struct FindOrderByIdCommand {
    pub id: Uuid,
}
