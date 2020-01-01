use pcm::todo_list::comment as ToDoComment;
use pcm::todo_list::description::*;
use pcm::todo_list_comment_create;
use pcm::todo_list_comment_update;

fn main() {
    println!("--- comment ----");
    let c_create = todo_list_comment_create(String::from("comment message"));
    let c_update = todo_list_comment_update(String::from("comment message"));
    let c_get_1 = ToDoComment::get(1);
    let c_get_2 = ToDoComment::get(2);

    println!("comment create -> {:#?}", c_create);
    println!("comment update -> {:#?}", c_update);
    println!("comment get 1 -> {:#?}", c_get_1);
    println!("comment get 2 -> {:#?}", c_get_2);

    println!("--- description ----");
    let d_update_1 = update(Some(1), String::from("description message"));
    let d_update_none = update(None, String::from("description message"));
    let d_get_1 = get(1);
    let d_get_2 = get(2);

    println!("description update 1 -> {:#?}", d_update_1);
    println!("description update none -> {:#?}", d_update_none);
    println!("description get 1 -> {:#?}", d_get_1);
    println!("description get 2 -> {:#?}", d_get_2);
}
