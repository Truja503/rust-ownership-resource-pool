fn main() {
    let resource_array = ["Water", "Food", "Medicine", "Tools", "Clothing", "Shelter", "Fuel", "Communication Devices", "First Aid Kits", "Sanitation Supplies"];

    let mut pool = Inventory{
        //add resources in the vector
        resources: Vec::new(),
    };

    //add some resources to the inventory pool, here is where the resources born
    for r in resource_array.iter(){
        pool.resources.push(Resources{
            name: String::from(*r),
        });
    }

    //User1, who can take resources from the pool
    let mut  user1 = Inventory{
        resources: Vec::new(), //move the resources to the user
    };
    //make 3 users more
    let mut user2 = Inventory{
        resources: Vec::new(),
    };
    //This create the scope for user3, this add items, and when the scope ends, the drop function is called
    {   
        let mut user3 = Inventory{
        resources: Vec::new(),
        };
        add_resource(&mut user3, &mut pool, "Medicine");
        println!("User3 inventory");
        print_historial(&user3);
    }
    let mut user4 = Inventory{
        resources: Vec::new(),
    };

    print_historial(&pool);
    println!("--- Users acquiring resources ---");    
    add_resource(&mut user1, &mut pool, "Food");
    add_resource(&mut user2, &mut pool, "Water");
    add_resource(&mut user1, &mut pool, "Tools");
    add_resource(&mut user4, &mut pool, "Clothing");

    println!("--- After acquiring resources ---");
    print_historial(&pool);

    release_resource(&mut user1, &mut pool, "Food");

    println!("User1 inventory");
    print_historial(&user1);        
    println!("User2 inventory");
    print_historial(&user2);
    //println!("User3 inventory"); this line is commented because user3 is out of scope now
    //print_historial(user3);
    println!("User4 inventory");
    print_historial(&user4);


}

struct Resources{
    name: String,
}

struct Inventory{
    resources: Vec<Resources>,
}

//drop for Resources
impl Drop for Resources{
    fn drop(&mut self){
        println!("Resource {} is being dropped", self.name);
    }
}

trait Acquire {
    fn acquire(&mut self,pool: &mut Inventory, resource_name: &str) -> Option<Resources>;
}

impl Acquire for Inventory{
    fn acquire(&mut self, pool: &mut Inventory, resource_name: &str) -> Option<Resources>{
        if let Some(pos) = pool.resources.iter().position(|r| r.name == resource_name){
            let resource = pool.resources.remove(pos);
            Some(resource)
        } else {
            None
        }
    }
}


//llamo al pool, porque será necesaria para la extracción del recurso
trait Release {
    fn release(&mut self,resource_name: &str) -> Option<Resources>;
}

impl Release for Inventory{
    fn release(&mut self, resource_name: &str) -> Option<Resources>{
        let matched_index = self.resources.iter().position(|r| r.name == resource_name);
        let resource = self.resources.remove(matched_index.unwrap());
        Some(resource)
    }
}



fn add_resource(user: &mut Inventory, pool: &mut Inventory, resource_name: &str){
    let resource = Acquire::acquire(user, pool, resource_name).unwrap();
    user.resources.push(resource);
}

fn release_resource(user1: &mut Inventory, pool: &mut Inventory, resource_name: &str){
    let resource = Release::release(user1, resource_name);
    pool.resources.push(resource.unwrap());
}

fn print_historial(pool: &Inventory){
    for r in pool.resources.iter(){
        println!("Resource: {}", r.name);
    }
}