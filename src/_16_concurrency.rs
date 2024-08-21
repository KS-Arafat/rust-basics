use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

pub fn spawningthread() {
    println!("[Thread]:");

    // We have to pass closure to thread::spawn to be executed in spawned thread
    let thread_handler = thread::spawn(|| {
        for i in 1..=3 {
            thread::sleep(Duration::from_millis(800));
            println!("Spawned Thread Prints: {i}");
        }
    });
    for i in 1..=3 {
        thread::sleep(Duration::from_millis(500));
        eprintln!("Main Thread Prints: {i}");
    }

    thread_handler.join().expect("Thread Exception");
    // This line above prevents main thread to died before spawned thread finishes
    // without the handler spawned threads might die prematurely
}

pub fn msg_passing() {
    println!("[MPSC]: Multi Producer Single Consumer");
    // mpsc is used to create channels tp send and receive values from
    // multiple threads without having to deal with ownership and blocking issues
    // But to use same data over multiple threads, clone has to be used
    let (tx, rx) = mpsc::channel::<Vec<i32>>();
    let data_across_threads = "This data Should be Accessed in all threads".to_string();
    let cloned_data = data_across_threads.clone();
    thread::spawn(move || {
        let mut prime_vec: Vec<i32> = vec![];
        drop(cloned_data);
        prime_vec.push(2);
        prime_vec.push(3);
        prime_vec.push(5);
        prime_vec.push(7);
        prime_vec.push(11);
        tx.send(prime_vec).expect("Error sending message");
    });
    let rx_vec = rx.recv().expect("Error recieving message");
    println!("Received Vector: {rx_vec:?}");
    /* Error:
     * println!("Received Vector: {prime_vec:?}");
     * borrow of moved value: `prime_vec`
     */

    // For multiple senders we can clone tx and iterate through rx in
    // for loop like vector and use them as we want
    /*
     * let tx_clone = tx.clone();
     * thread::spawn(move || tx.send("Tx 1"));
     * thread::spawn(move || tx_clone.send("Tx 2"));
     * for r in rx{
     *   println!("Received {r}");
     * }
     *
     */
}

// when we use variable in thead closure and we make some changes to that,
// we have to pass the ownership to the closure or it won't work
// also passed ownership invalidates the variable from further use in main thread
// So we will see some APIs that solves this problem
pub fn mutual_exclusion() {
    println!("[Mutex]: Mutual Exclusion");
    let mux_vec = Mutex::new(vec![] as Vec<i32>);
    // In Mutex, only single thread can make changes to the mutex variable
    // .lock() method locks the mutex variable from changes by other threads
    // if mutex is locked then .unwrap() will make wait for to be locked
    mux_vec.lock().unwrap().push(5);
    {
        let mut scope_vec = mux_vec.lock().unwrap();
        scope_vec.push(7);
    }
    println!("Mutex Vector: {:?}", mux_vec.lock().unwrap());
}

pub fn atomic_ref_count() {
    // Arc(Atomic Reference Counter) is type of Rc(Reference Counter) Struct
    // that is safe for threads to access references across multiple threads
    // Rc is for single thread access and Arc is for multiple threads access

    println!("[Arc]: Atomic Reference Counter");
    let arc_counter = Arc::new("Arc-Counter".to_string());

    // Whenever we've to move an ownership,
    // we have to create Arc clone of that reference
    let clone_1 = Arc::clone(&arc_counter);
    let clone_2 = Arc::clone(&arc_counter);

    let mut thread_handlers = vec![];
    thread_handlers.push(thread::spawn(move || {
        // {:p} is for printing variable memory address
        println!("Spawned Thread 01: \t{}, Addr: {:p}", clone_1, clone_1)
    }));
    thread_handlers.push(thread::spawn(move || {
        println!("Spawned Thread 02: \t{}, Addr: {:p}", clone_2, clone_2)
    }));

    for thread in thread_handlers {
        thread.join().expect("Thread Error:");
    }
    println!("Main Thread: \t\t{:}, Addr: {:p}", arc_counter, arc_counter);
}

pub fn arc_mutex_chain() {
    println!("[Arc & Mutex]: Creating Mutable Ref across Threads");

    // Arc is alone immutable, so we have to make use of mutex and arc for
    // safe reference access across all threads

    let arc_counter = Arc::new(Mutex::new(vec![] as Vec<i32>));
    let mut handles = vec![];

    println!("Mutex Locked State:");
    for i in 1..=20 {
        let arc_clone = Arc::clone(&arc_counter);
        let handler = thread::spawn(move || {
            // try_lock() tests if mutex is locked or not,
            // Ok arg returns mutex value
            // err return blocked message
            match arc_clone.try_lock() {
                Ok(_lock) => println!("Thread {i}\tUnlocked"),
                Err(_err) => println!("Thread {i}\tLocked"),
            }
            // Lock the mutex before accessing the data
            let mut mut_vec = arc_clone.lock().unwrap();
            mut_vec.push(i);
            // Mutex lock is automatically released when 'mut_vec' goes out of scope
        });
        handles.push(handler);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // After all threads have finished, print the result
    println!("Result: {:?}", arc_counter.lock().unwrap());
}
