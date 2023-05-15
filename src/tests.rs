macro_rules! repeat {
    ($f: ident, $t: ident) => {
        #[test]
        fn $f () {
            let mut v = super::BitVec::<$t>::new();

            v.resize(32);
    
            v.set(0, true);
            v.set(2, true);
            v.set(31, true);

            v.resize(64);
            
            v.set(32, true);
            v.set(34, true);
            v.set(63, true);

            v.resize(128);

            v.set(127, true);

            println!("{:#?}", v);

            assert!(v.get(32) == true);
            assert!(v.get(33) == false);
            assert!(v.get(127) == true);

            v.resize(32);
            
            println!("{:#?}", v);
        }
    }
}

repeat!(test_u8, u8);
repeat!(test_u16, u16);
repeat!(test_u32, u32);
repeat!(test_u64, u64);
repeat!(test_u128, u128);