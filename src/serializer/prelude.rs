pub use super::category::Category;
pub use super::friend::Friend;
pub use super::id::ID;
pub use super::jwt::Jwt;
pub use super::list::List;
pub use super::post::Post;

pub type PostList = List<Post>;
pub type CateList = List<Category>;
pub type FriendList = List<Friend>;
