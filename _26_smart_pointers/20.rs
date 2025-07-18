// This example showcases a recursive data structure, `FileSystemEntity`, that
// models a file system with files and folders. A `Folder` variant can contain a
// `Vec<FileSystemEntity>`, making the enum recursive. Surprisingly, this code
// compiles without requiring an explicit `Box<T>` or other forms of indirection.
// The reason is that `Vec<T>` is itself a smart pointer. A vector stores its
// elements on the heap, while the `Vec` struct itself, which lives on the stack,
// has a fixed, known size (containing a pointer to the heap data, its length,
// and its capacity). This inherent indirection provided by the vector prevents
// the infinite-size compilation error that would otherwise occur with recursive
// types. `Vec<T>` manages the memory of its contents, abstracting away the
// complexity and behaving like an owned type, similar to `Box<T>`.

#[derive(Debug)]
enum FileSystemEntity {
    File {
        name: String,
    },
    Folder {
        name: String,
        content: Vec<FileSystemEntity>
    },
}

fn main() {
    let rust_file = FileSystemEntity::File {
        name: String::from("my_rust_code.rs")
    };
    let python_file = FileSystemEntity::File {
        name: String::from("my_python_code.py")
    };
    let code_folder = FileSystemEntity::Folder {
        name: String::from("Code Stuff"),
        content: vec![rust_file, python_file]
    };
    let screenplay = FileSystemEntity::File {
        name: String::from("My screenplay.txt")
    };
    let all_documents = FileSystemEntity::Folder {
        name: String::from("Documents"),
        content: vec![screenplay, code_folder]
    };

    println!("{all_documents:#?}");
}