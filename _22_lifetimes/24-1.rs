// Lecture: When multiple lifetimes are actually needed
// Real-world scenario: A web request handler that needs to reference
// both configuration data (lives for the entire program) and request data
// (lives only for the duration of the request)

#[derive(Debug)]
struct RequestHandler<'config, 'request> {
    server_name: &'config str,    // Lives for entire program
    request_path: &'request str,  // Lives only during request processing
}

impl<'config, 'request> RequestHandler<'config, 'request> {
    // This function shows why we need separate lifetimes:
    // - It can return the server_name (long-lived)
    // - Or the request_path (short-lived)
    // - The caller needs to know which lifetime they're getting
    fn get_server_info(&self) -> &'config str {
        self.server_name  // Returns long-lived reference
    }
    
    fn get_request_info(&self) -> &'request str {
        self.request_path  // Returns short-lived reference
    }
    
    // This function demonstrates the real need: it chooses which reference
    // to return based on some logic, and each has a different lifetime
    fn get_info_for_logging(&self, include_path: bool) -> (&'config str, Option<&'request str>) {
        if include_path {
            (self.server_name, Some(self.request_path))
        } else {
            (self.server_name, None)
        }
    }
}

fn main() {
    // Configuration data - lives for the entire program
    let server_config = String::from("MyWebServer v1.0");
    
    // This represents the server info that can be used across many requests
    let server_info: &str;
    
    {
        // Simulate processing a request - this data lives only during request processing
        let incoming_request = String::from("/api/users/profile");
        
        let handler = RequestHandler {
            server_name: &server_config,     // Long lifetime
            request_path: &incoming_request, // Short lifetime
        };
        
        // We can safely extract the server name because it outlives this scope
        server_info = handler.get_server_info();
        
        // We can get request info, but only use it within this scope
        let request_info = handler.get_request_info();
        println!("Processing request: {} on {}", request_info, server_info);
        
        // This demonstrates the different lifetimes in action
        let (server, maybe_path) = handler.get_info_for_logging(true);
        println!("Log: Server={}, Path={:?}", server, maybe_path);
        
        // request_info and maybe_path become invalid after this scope ends
    }
    
    // server_info is still valid here because it references long-lived data
    println!("Server still running: {}", server_info);
    
    // This would NOT compile if we tried to use request_info here:
    // println!("Request was: {}", request_info); // ERROR: request_info not in scope
}

// Why multiple lifetimes are essential here:
// 1. The server_name needs to live as long as the server runs
// 2. The request_path only needs to live during request processing  
// 3. Functions need to specify which lifetime they return
// 4. This allows the borrow checker to enforce correct usage:
//    - Long-lived references can be stored and used later
//    - Short-lived references must be consumed quickly
// 5. Without separate lifetimes, we'd either:
//    - Make everything short-lived (inefficient, limits reuse)
//    - Make everything long-lived (unsafe, potential dangling references)

// Multiple lifetimes aren't just about "not knowing which reference will be returned." They're about references that have fundamentally different lifespans and need to be tracked separately.
// In our example:
// server_name references data (server_config) that lives for the entire program
// request_path references data (incoming_request) that only lives during request processing
// The key insight: Even when a function returns both references (like get_info_for_logging), the borrow checker needs to know that:
// The first part of the tuple (&'config str) can be used later
// The second part (Option<&'request str>) can only be used within the current scope