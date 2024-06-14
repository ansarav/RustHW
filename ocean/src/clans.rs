use std::collections::HashMap;
#[derive(Debug)]

pub struct ClanSystem {
    clans : HashMap<String, Vec<String>>,
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
        let mut largest_clan_id = None;
        let mut largest_size = 0 ;

        for (clan_id ,members) in &self.clans{
            let size_here = members.len();
            if size_here > largest_size {
                largest_clan_id = Some(clan_id.clone());
                largest_size = size_here;
            }
        }

        largest_clan_id
    }

    pub fn add_member(&mut self, clan_id: &str, name: &str){
        if let Some(clan) = self.clans.get_mut(clan_id){
            clan.push(name.to_string());
        }
        else{
            let mut new_clan = Vec::new();
            new_clan.push(name.to_string());
            self.clans.insert(clan_id.to_string(), new_clan);
        }
    }
}