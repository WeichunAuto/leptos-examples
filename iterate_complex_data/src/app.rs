use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet};
use reactive_stores::{Store, StoreFieldIterator};

/// The top level of a store always needs to be a struct, so we’ll create a Data wrapper with a single rows field.
#[derive(Debug, Clone, Store)]
pub struct DataStore {
    #[store(key: String = |row| row.key.clone())]
    pub rows: Vec<Student>,
}

#[derive(Debug, Clone, Store)]
pub struct Student {
    pub key: String,
    pub name: String,
    pub height: u16,
}
impl Student {
    pub fn new(key: String, name: String, height: u16) -> Self {
        Self { key, name, height }
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let s1 = Student::new("000".to_string(), "Lucy".to_string(), 175);
    let s2 = Student::new("111".to_string(), "Mali".to_string(), 180);
    let s3 = Student::new("222".to_string(), "Bob".to_string(), 178);

    let data_store = Store::new(DataStore {
        rows: vec![s1, s2, s3],
    });

    // 新增
    let add_student = move |_| {
        // let rows_vec = data_store.rows().read();  // 获取读取锁, 锁释放之前调用write()写锁，会panic

        let current_size = data_store.rows().read().len(); // read().len()之后读锁会释放
        let new_key = current_size.to_string();

        let new_student = Student::new(new_key, format!("Student {}", current_size + 1), 180);

        data_store.rows().write().push(new_student);
    };

    // 删除
    let delete_student = move |index: usize| {
        let rows_field = data_store.rows(); // .rows()返回一个临时值，所以不可以直接.rows().write()
        let mut rows = rows_field.write();
        if let Some(pos) = rows.iter().enumerate().position(|(i, _)| i == index) {
            let s_remove: &Student = rows.get(pos).unwrap();
            leptos::logging::log!("删除了学生: {:?}", *s_remove);
            rows.remove(pos);
        }
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/iterate_complex_data.css"/>

        <h1>"Complex data iteration"</h1>

        <button on:click = move |_| {
            for row_signal in data_store.rows().iter_unkeyed() { // 遍历所有元素，不带 index
                let height = row_signal.height();
                leptos::logging::log!("{:?}", height.get());
                *height.write() *= 2;
            }
        }>
            "更新学生身高"
        </button>

        <button on:click= move |_| {
            let special_name = "Bob";
            for row_signal in data_store.rows().iter_unkeyed() {
                let name_signal = row_signal.name();
                let current_name = name_signal.get();
                let new_name = format!("{} SUPER", current_name);

                if current_name.eq(special_name) {
                    *name_signal.write() = format!("Mr {}", new_name);
                } else {
                    *name_signal.write() = new_name;
                }
            }
        }>"更新姓名"</button>

        <button on:click= add_student>"添加学生"</button>

        <ul>
            <ForEnumerate
                each = move || data_store.rows() // .rows() 是个getter 方法
                key = |row_signal| row_signal.read().key.clone()
                children = move |index, row_signal| {
                    view! {
                        <li>{move || {
                            let student = row_signal.read(); // 一次性提取 student 对象
                            format!("index: {}, key: {}, name: {}, height: {} ", index.get(), student.key, student.name, student.height)

                        }}
                        <button on:click= move |_| {
                             delete_student(index.get());
                        }>"删除学生"</button>
                        </li>
                    }
                }
            >

            </ForEnumerate>

        </ul>
    }
}
