fn main() {
    // 1. Variable Scope
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // 2. The String Type
    {
        let mut s = String::from("hello");

        s.push_str(", world!"); // push_str() appends a literal to a String

        println!("{}", s); // This will print `hello, world!
    }

    // 3. Memory and Allocation
    {
        {
            let s = String::from("hello"); // s is valid from this point forward

            // do stuff with s
        } // this scope is now over, and s is no
        // longer valid
    }

    // 4. Ways Variables and Data Interact: Move
    {
        let s1 = String::from("hello");
        let s2 = s1;
    
        //println!("{}, world!", s1); // This will not work
    }

    // 5. Ways Variables and Data Interact: Clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
    
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    // 6. Ways Variables and Data Interact: Copy
    {
        let x = 5;
        let y = x;
    
        println!("x = {}, y = {}", x, y);
    }
}
