/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = crates::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// A struct with name and age fiels
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    /// Instantiates a new instance of person with default values
    ///
    /// # Examples
    /// ```
    /// let person = crates::Person::new();
    /// assert_eq!(person.name, "Default");
    /// assert_eq!(person.age, 1)
    /// ```
    pub fn new() -> Self {
        Person {
            name: "Default".to_string(),
            age: 1,
        }
    }
}
