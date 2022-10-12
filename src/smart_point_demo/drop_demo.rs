struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("釋放 CustomSmartPointer 的資料 `{}`!", self.data);
    }
}

/** 
 * Rust 不讓我們顯式呼叫 drop，因為 Rust 還是會在 main 結束時自動呼叫 drop。這樣可能會導致重複釋放（double free）的錯誤，因為 Rust 可能會嘗試清除相同的數值兩次。
 * 當數值離開作用域時我們無法停用自動插入的 drop，而且我們無法顯式呼叫 drop 方法，所以如果我必須強制讓一個數值提早清除的話，我們可以用 std::mem::drop 函式。
 */
pub fn drop_demo() {
    let c = CustomSmartPointer {
        data: String::from("my code")
    };
    println!("CustomSmartPointer 建立完畢。");
    // c.drop(); // explicit destructor calls not allowed
    drop(c);
    println!("CustomSmartPointer 在 main 結束前就被釋放了。");
}