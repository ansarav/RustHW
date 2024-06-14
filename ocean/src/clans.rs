
#[derive(Debug)]
pub struct ClanSystem {
    clans : HashMap<String, HashSet<String>>,
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        ClanSystem{
            clans: HashMap::new(),
        }
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        //if let
        match self.clans.get(clan_id){
            Some(member) => member.clone(),
            None => Vec ::new(),
        }
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        self.clans.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        match self.clans.get(clan_id){
            Some(member) => member.len(),
            None => 0
        }
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        //similar format to get_fastest_crab
        let mut largets_clan_id = None;
        let largest_size = 0 ;

        for (clan_id ,members) in &self.clans{
            let mem_count = members.len();
            if mem_count > largets_clan_id {
                largets_clan_id = Some(clan_id.clone());
                largest_size = mem_count
            }
        }

        largets_clan_id
    }
}