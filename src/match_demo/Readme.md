父模塊和子模塊的包含方法
===

有時候一個 folder 裏面有個 mod.rs，有時候沒有。

這是因爲 mod foo 的作用是告訴編譯器找尋 foo.rs 或者 foo/mod.rs，並且將找尋到的文件內容作爲 module foo 的內容。

你可以選擇你喜歡的方式使用 `foo.rs` 或者 `foo/mod.rs`。

> 使用文件夾 foo 存儲 modules 的時候，我們可以創建一個跟文件夾同名的 foo.rs 來添加文件夾裏面的 modules，也可以在文件夾 foo 裏面用 mod.rs 添加對應的 modules。

所以，mod foo 是將 foo 模塊添加到 project 的 module tree。

use foo; 是將一個模塊加進當前的 scope。

而./foo.rs 或者./foo/mod.rs, 或者 mod foo {...} 定義模塊，它們三者是等效的。