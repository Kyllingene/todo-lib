use crate::*;

#[test]
/// Tests creation, addition, completion, and moving of todos and tables.
fn create_add_complete_move() {
    let mut todos = TodoTable::new::<String>(None);
    todos.add_col("A");
    todos.add_col("B");

    todos.add_todo(Todo::new("1", TodoDate::Never, TodoPriority::None), "A");
    todos.add_todo(Todo::new("2", TodoDate::Never, TodoPriority::None), "B");

    let todo1 = todos.get_todo("1", "A");
    assert!(todo1.is_some(), "Failed to retrieve todo 1");

    todo1.unwrap().complete();

    assert!(todos.move_todo("2", "B", "A"));

    let todo2 = todos.get_todo("2", "A");
    assert!(todo2.is_some(), "Failed to retrieve todo 2");

    todo2.unwrap().complete();
}

#[test]
/// Tests due dates.
fn is_due() {
    let todo = Todo::new(
        "",
        TodoDate::Day(Local::now().naive_local()),
        TodoPriority::None,
    );
    assert!(
        todo.due(),
        "Should be due, isn't; deadline{}",
        todo.deadline
    );

    let todo = Todo::new("", TodoDate::Always, TodoPriority::None);
    assert!(
        todo.due(),
        "Should be due, isn't; deadline{}",
        todo.deadline
    );

    let todo = Todo::default();
    assert!(
        !todo.due(),
        "Shouldn't be due, is; deadline{}",
        todo.deadline
    );
}

#[test]
fn todo_display() {
    let mut todo = Todo::new("Todo #1", TodoDate::Always, TodoPriority::B);

    let today = Local::now();
    assert_eq!(
        todo.to_string(),
        format!(
            "(B) {}-{:02}-{:02} Todo #1 due:0000-00-00",
            today.year(),
            today.month(),
            today.day()
        )
    );

    todo.complete();
    assert_eq!(
        todo.to_string(),
        format!(
            "x (B) {0}-{1:02}-{2:02} {0}-{1:02}-{2:02} Todo #1 due:0000-00-00",
            today.year(),
            today.month(),
            today.day()
        )
    );
}

#[test]
fn tag_parsing() {
    assert!(
        TodoTag::project("a ").is_none(),
        "TodoTag::project accepted space"
    );
    assert!(
        TodoTag::project("b\n").is_none(),
        "TodoTag::project accepted newline"
    );
    assert!(
        TodoTag::project("c\t").is_none(),
        "TodoTag::project accepted tab"
    );

    assert!(
        TodoTag::context("a ").is_none(),
        "TodoTag::context accepted space"
    );
    assert!(
        TodoTag::context("b\n").is_none(),
        "TodoTag::context accepted newline"
    );
    assert!(
        TodoTag::context("c\t").is_none(),
        "TodoTag::context accepted tab"
    );

    let a = Todo::new(
        "+1 @2 3+4 @a +b-c @def@ghi jk@lm",
        TodoDate::Never,
        TodoPriority::None,
    );
    let tags = a.tags();

    assert!(
        tags.contains(&TodoTag::project("1").unwrap()),
        "Couldn't find +1"
    );
    assert!(
        tags.contains(&TodoTag::context("2").unwrap()),
        "Couldn't find @2"
    );
    assert!(
        !tags.contains(&TodoTag::project("4").unwrap()),
        "Shouldn't find +4"
    );
    assert!(
        tags.contains(&TodoTag::context("a").unwrap()),
        "Couldn't find @a"
    );
    assert!(
        tags.contains(&TodoTag::project("b-c").unwrap()),
        "Couldn't find +b-c"
    );
    assert!(
        tags.contains(&TodoTag::context("def@ghi").unwrap()),
        "Couldn't find @def@ghi"
    );
    assert!(
        !tags.contains(&TodoTag::context("lm").unwrap()),
        "Shouldn't find @lm"
    );
}

#[test]
fn todo_txt_parsing() {
    let todo_text = "2023-01-07 Create a +todo @library due:2053-01-01";
    let mut todo = Todo::from_str(todo_text).unwrap();

    assert_eq!(todo.to_string(), todo_text);
    assert!(!todo.due());

    todo.completed = true;

    assert_eq!(
        todo.to_string(),
        "x 2023-01-07 Create a +todo @library due:2053-01-01"
    );

    assert!(todo.has_project_tag("todo"));
    assert!(todo.has_context_tag("library"));
}

#[test]
fn todo_txt_metadata() {
    let todo_text = "2023-01-16 Add metadata to the +todo @library due:2000-01-01 exam:ple";
    let mut todo = Todo::from_str(todo_text).unwrap();

    assert!(
        todo.due(),
        "Should be due, isn't; duedate: {:?}",
        todo.deadline
    );
    assert_eq!(todo.get_meta(&"exam".to_string()), Some(&"ple".to_string()),);

    todo.delete_meta(&"exam".to_string());
    todo.set_meta("key".to_string(), "val".to_string());

    assert_eq!(
        todo.to_string(),
        "2023-01-16 Add metadata to the +todo @library due:2000-01-01 key:val",
    );
}
