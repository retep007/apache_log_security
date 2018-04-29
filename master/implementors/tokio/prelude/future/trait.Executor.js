(function() {var implementors = {};
implementors["tokio"] = ["impl&lt;F&gt; <a class=\"trait\" href=\"tokio/prelude/future/trait.Executor.html\" title=\"trait tokio::prelude::future::Executor\">Executor</a>&lt;F&gt; for <a class=\"struct\" href=\"tokio/executor/current_thread/struct.TaskExecutor.html\" title=\"struct tokio::executor::current_thread::TaskExecutor\">TaskExecutor</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"tokio/prelude/future/trait.Future.html\" title=\"trait tokio::prelude::future::Future\">Future</a>&lt;Item = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, Error = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt; + 'static,&nbsp;</span>","impl&lt;T&gt; <a class=\"trait\" href=\"tokio/prelude/future/trait.Executor.html\" title=\"trait tokio::prelude::future::Executor\">Executor</a>&lt;T&gt; for <a class=\"struct\" href=\"tokio/runtime/struct.TaskExecutor.html\" title=\"struct tokio::runtime::TaskExecutor\">TaskExecutor</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"tokio/prelude/future/trait.Future.html\" title=\"trait tokio::prelude::future::Future\">Future</a>&lt;Item = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, Error = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,&nbsp;</span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
