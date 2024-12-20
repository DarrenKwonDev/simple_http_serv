use std::collections::HashMap;

//  구조체 안에 저장하는 모든 참조에 대해 수명을 명시적으로 지정해야 합니다.
#[derive(Debug)]

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

#[derive(Debug)]

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            // a=1이 왔다고 생각해보라.
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i]; // a
                val = &sub_str[i+1..]; // 1
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val, val]); 
                    },
                    Value::Multiple(vec) => vec.push(&val)
                })
                .or_insert(Value::Single(val));
        }

        QueryString {data}
    }
}
