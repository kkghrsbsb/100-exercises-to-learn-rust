// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
//
// TODO：定义一个新的 `Order` 类型。
//  它应该跟踪三个信息：`product_name`、`quantity` 和 `unit_price`。
//  产品名称不能为空，且长度不能超过 300 字节。
//  数量必须严格大于零。
//  单价以美分为单位，且必须严格大于零。
//  Order 类型必须包含一个名为 `total` 的方法，该方法返回订单的总价。
//  Order 类型必须为每个字段提供 setter 和 getter 方法。
//
// 这次测试文件位于不同的位置——在 `tests` 文件夹中。
// `tests` 文件夹是 `cargo` 的一个特殊位置。它会在这里查找**集成测试**。
// 这里的“集成”有非常具体的含义：它们测试项目的**公共 API**。
// 你需要注意类型和方法的可见性；集成
// 测试无法访问私有或 `pub(crate)` 项。

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        Self::validate_product_name(&product_name);
        Self::validate_quantity(quantity);
        Self::validate_unit_price(unit_price);

        Self {
            product_name,
            quantity,
            unit_price
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, new_product_name: String) {
        Self::validate_product_name(&new_product_name);
        self.product_name = new_product_name;
    }

    pub fn set_quantity(&mut self, new_quantity: u32) {
        Self::validate_quantity(new_quantity);
        self.quantity = new_quantity;
    }

    pub fn set_unit_price(&mut self, new_unit_price: u32) {
        Self::validate_unit_price(new_unit_price);
        self.unit_price = new_unit_price;
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    fn validate_product_name(product_name: &String) {
        if product_name.is_empty() {
            panic!();
        }
        if product_name.len() > 300 {
            panic!();
        }
    }

    fn validate_quantity(quantity: u32) {
        if quantity <= 0 {
            panic!();
        }
    }

    fn validate_unit_price(unit_price: u32) {
        if unit_price <= 0 {
            panic!();
        }
    }
}