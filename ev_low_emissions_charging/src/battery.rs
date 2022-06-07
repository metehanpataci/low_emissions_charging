


pub struct Battery{

    pub current_state : u8, // Current state of the charge. Percentage

    pub capacity : u32, // Battery Capacity of kWh

    pub phase_count : u16, // Phase count

    pub amper : u16, // Amper value

    pub volt : u16, // Volt

    pub efficiency : f32, // Charging efficiency. Meaning that each kWh from the grid delivers. ex: 90% means 0.9kWh in the battery

}

impl Battery{

    fn new()-> Self{

        Self{
            current_state : 0,
            capacity : 60, 
            phase_count : 3,
            amper : 16,
            volt:230,
            efficiency : 0.9,
        }

    }

    pub fn get_current_state(&self) -> u8
    {
        self.current_state
    }

    pub fn get_capacity(&self) -> u32
    {
        self.capacity
    }

    pub fn get_phase_count(&self) -> u16
    {
        self.phase_count
    }

    pub fn get_amper(&self) -> u16
    {
        self.amper
    }

    pub fn get_volt(&self) -> u16
    {
        self.volt
    }

    pub fn get_efficiency(&self) -> f32
    {
        self.efficiency
    }

}