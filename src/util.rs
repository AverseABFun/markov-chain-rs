pub fn compare_const_strs(ptr1: *const str, ptr2: *const str) -> bool {
    unsafe {
        if ptr1.is_null() || ptr2.is_null() {
            return false;
        }

        let str1: &str = &*ptr1;
        let str2: &str = &*ptr2;

        str1 == str2
    }
}
