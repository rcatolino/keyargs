    testlib::test(3, ::std::option::None, ::std::option::None);
    testlib::test("mandatory", ::std::option::None, ::std::option::None);
    testlib::test("mandatory", ::std::option::Some("opt"),
                  ::std::option::None);
    testlib::test("mandatory", ::std::option::Some("opt"),
                  ::std::option::Some(3));
    testlib::test("mandatory", ::std::option::Some("opt"),
                  ::std::option::Some(3));
    testlib::test("mandatory", ::std::option::Some("opt"),
                  ::std::option::Some(3));
    testlib::test("mandatory", ::std::option::Some("opt"),
                  ::std::option::Some(3));
