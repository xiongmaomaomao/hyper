(function() {var implementors = {};
implementors['time'] = ["<a class='stability Experimental' title='Experimental'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/ops/trait.Add.html' title='core::ops::Add'>Add</a>&lt;<a class='struct' href='http://doc.rust-lang.org/nightly/std/time/duration/struct.Duration.html' title='std::time::duration::Duration'>Duration</a>, <a class='struct' href='time/struct.Timespec.html' title='time::Timespec'>Timespec</a>&gt; for <a class='struct' href='time/struct.Timespec.html' title='time::Timespec'>Timespec</a>","<a class='stability Experimental' title='Experimental'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/ops/trait.Add.html' title='core::ops::Add'>Add</a>&lt;<a class='struct' href='http://doc.rust-lang.org/nightly/std/time/duration/struct.Duration.html' title='std::time::duration::Duration'>Duration</a>, <a class='struct' href='time/struct.Tm.html' title='time::Tm'>Tm</a>&gt; for <a class='struct' href='time/struct.Tm.html' title='time::Tm'>Tm</a>",];
implementors['openssl'] = ["<a class='stability Experimental' title='Experimental'></a>impl&lt;'a&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/ops/trait.Add.html' title='core::ops::Add'>Add</a>&lt;&amp;'a <a class='struct' href='openssl/bn/struct.BigNum.html' title='openssl::bn::BigNum'>BigNum</a>, <a class='struct' href='openssl/bn/struct.BigNum.html' title='openssl::bn::BigNum'>BigNum</a>&gt; for &amp;'a <a class='struct' href='openssl/bn/struct.BigNum.html' title='openssl::bn::BigNum'>BigNum</a>",];
implementors['hyper'] = ["<a class='stability Experimental' title='Experimental'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/ops/trait.Add.html' title='core::ops::Add'>Add</a>&lt;<a class='struct' href='http://doc.rust-lang.org/nightly/std/time/duration/struct.Duration.html' title='std::time::duration::Duration'>Duration</a>, <a class='struct' href='http://doc.rust-lang.org/nightly/std/time/duration/struct.Duration.html' title='std::time::duration::Duration'>Duration</a>&gt; for <a class='struct' href='http://doc.rust-lang.org/nightly/std/time/duration/struct.Duration.html' title='std::time::duration::Duration'>Duration</a>","<a class='stability Experimental' title='Experimental: waiting on Add stabilization'></a>impl&lt;'a&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/ops/trait.Add.html' title='core::ops::Add'>Add</a>&lt;&amp;'a <a href='http://doc.rust-lang.org/nightly/std/primitive.str.html'>str</a>, <a class='struct' href='http://doc.rust-lang.org/nightly/collections/string/struct.String.html' title='collections::string::String'>String</a>&gt; for <a class='struct' href='http://doc.rust-lang.org/nightly/collections/string/struct.String.html' title='collections::string::String'>String</a>","<a class='stability Experimental' title='Experimental'></a>impl&lt;'a, T: <a class='trait' href='http://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a>&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/ops/trait.Add.html' title='core::ops::Add'>Add</a>&lt;<a href='http://doc.rust-lang.org/nightly/std/primitive.slice.html'>&amp;'a [T]</a>, <a class='struct' href='http://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html' title='collections::vec::Vec'>Vec</a>&lt;T&gt;&gt; for <a class='struct' href='http://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html' title='collections::vec::Vec'>Vec</a>&lt;T&gt;",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
