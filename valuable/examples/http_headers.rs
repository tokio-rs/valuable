use valuable::*;

#[derive(Valuable)]
struct Headers {
    user_agent: String,
    host: String,
    content_type: ContentType,
    accept_encoding: Vec<String>,
}

#[derive(Valuable)]
struct ContentType {
    mime: String,
    charset: String,
}

// Visit the root of the Headers struct. This visitor will find the
// `accept_encoding` field on `Headers` and extract the contents. All other fields
// are ignored.
struct VisitHeaders {
    /// The extracted `accept-encoding` header values.
    accept_encoding: Vec<String>,
}

// Visit the `accept-encoding` `Vec`. This visitor iterates the items in the
// list and pushes it into its `accept_encoding` vector.
struct VisitAcceptEncoding<'a> {
    accept_encoding: &'a mut Vec<String>,
}

impl Visit for VisitHeaders {
    fn visit_value(&mut self, value: Value<'_>) {
        // We expect a `Structable` representing the `Headers` struct.
        match value {
            // Visiting the struct will call `visit_named_fields`.
            Value::Structable(v) => v.visit(self),
            // Ignore other patterns
            _ => {}
        }
    }

    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        // We only care about `accept_encoding`
        match named_values.get_by_name("accept_encoding") {
            Some(Value::Listable(accept_encoding)) => {
                // Create the `VisitAcceptEncoding` instance to visit the items
                // in `Listable`.
                let mut visit = VisitAcceptEncoding {
                    accept_encoding: &mut self.accept_encoding,
                };
                accept_encoding.visit(&mut visit);
            }
            _ => {}
        }
    }
}

impl Visit for VisitAcceptEncoding<'_> {
    /// Method called when visiting a `Listable`.
    fn visit_value(&mut self, value: Value<'_>) {
        if let Some(accept_encoding) = value.as_str() {
            self.accept_encoding.push(accept_encoding.to_string())
        }
    }
}

fn main() {
    let headers = Headers {
        user_agent: "Mozilla/4.0 (compatible; MSIE5.01; Windows NT)".to_string(),
        host: "www.example.com".to_string(),
        content_type: ContentType {
            mime: "text/xml".to_string(),
            charset: "utf-8".to_string(),
        },
        accept_encoding: vec!["gzip".to_string(), "deflate".to_string()],
    };

    // Extract the "accept-encoding" headers
    let mut visit = VisitHeaders { accept_encoding: vec![] };
    valuable::visit(&headers, &mut visit);

    assert_eq!(&["gzip", "deflate"], &visit.accept_encoding[..]);
}
