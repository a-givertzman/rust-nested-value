#[cfg(test)]

mod multi_value {
    use indexmap::IndexMap;
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{const_value::ConstValue, multi_value::MultiValue, mut_value::MutValue, nested_value::NestedValue, tests::value::Value};
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing MultiValue, ConstValue, MutValue get / store methods
    #[test]
    fn nested() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            ("u64", Value::U64(1234567890)),
            ("i64", Value::I64(-1234567890)),
            ("f64", Value::F64(12345.6789012345)),
            ("v1/f64", Value::F64(111.111111)),
            ("v1/v2/f64", Value::F64(222.222222)),
            ("v1/v2/vec", Value::Vec(vec![Value::U64(222), Value::I64(-222), Value::F64(222.222222)])),
            ("v1/v2/map", Value::Map(IndexMap::from([
                (Value::from("222"), Value::U64(222)),
                (Value::from("-222"), Value::I64(-222)),
                (Value::from("222.222"), Value::F64(222.222222)),
            ]))),
            // ("v1/v2/fetch-222", ),
            // ("v1/v2/fetch-222.222", ),
            // ("v1/fetch-vec", ),
            // ("v1/fetch-map", ),
            // ("v1/fetch-map", ),
        ];
        let target = Value::U64(1234567890);
        let value = ConstValue::new(target.clone());
        let result = value.get();
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);

        let mut value = MultiValue::new([
            ("u64", Box::new(ConstValue::new(Value::U64(1234567890)))),
            ("i64", Box::new(ConstValue::new(Value::I64(-1234567890)))),
            ("f64", Box::new(ConstValue::new(Value::F64(12345.6789012345)))),
            ("v1", Box::new(MultiValue::new([
                ("u64", Box::new(ConstValue::new(Value::U64(111)))),
                ("i64", Box::new(ConstValue::new(Value::I64(-111)))),
                ("f64", Box::new(ConstValue::new(Value::F64(111.111111)))),
                ("v2", Box::new(MultiValue::new([
                    ("u64", Box::new(ConstValue::new(Value::U64(222)))),
                    ("i64", Box::new(ConstValue::new(Value::I64(-222)))),
                    ("f64", Box::new(MutValue::new(Value::F64(222.222222)))),
    
                    ("vec", Box::new(ConstValue::new(Value::Vec(vec![Value::U64(222), Value::I64(-222), Value::F64(222.222222)])))),
    
                    ("map", Box::new(ConstValue::new(Value::Map(IndexMap::from([
                        (Value::from("222"), Value::U64(222)),
                        (Value::from("-222"), Value::I64(-222)),
                        (Value::from("222.222"), Value::F64(222.222222)),
                    ]))))),
                ]))),
            ]))),
        ]);
        for (key, target) in test_data {
            let result = value.get_(key).unwrap();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        let key = "v1/v2/f64";
        let target = Value::F64(333.333333);
        value.store("me", key, target.clone()).unwrap();
        let result = value.get_(key).unwrap();
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
    test_duration.exit();
    }
}
