use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::collections;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
     // we want to get access to all the crabs in beach
    all_crabs : Vec<Crab>,
    clans: ClanSystem

}

impl Beach {
    pub fn new() -> Beach {
        Beach{all_crabs: Vec::new(), clans: ClanSystem::new()}
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.all_crabs.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.all_crabs.push(crab);
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        // do NOT take ownership here, we just want to borrow so i'll use "&"
        &self.all_crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.all_crabs.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        let mut fastest_crab = None;
        let mut fastest_speed = 0;
        for ind_crab in &self.all_crabs {
            if ind_crab.speed() > fastest_speed {
                fastest_crab = Some(ind_crab);
                fastest_speed = ind_crab.speed();
            }
        }
        fastest_crab
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        // need to return Vector
        //use filter and then out into a collection all the crabs that have the same name

        let crab_iterator = self.crabs();
        let filter_names = crab_iterator.filter(|crab| crab.name() == name);
        let collections_crabs: Vec<&Crab> = filter_names.collect();

        collections_crabs
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        //get_mut from IndexMut 
        // if let Some(mut parent_i ) = self.all_crabs.get_mut(i){
        //     if let Some(parent_j) = self.all_crabs.get(j){

        //         let childs_color = Color::cross(parent_i.color(), parent_j.color());
        //         let childs_diet = Diet::random_diet();
        //         let child_born = Crab::new(name,1,childs_color,childs_diet);
        //         self.add_crab(child_born);
        //     }
        //     else{
        //         panic!("Index j out of bounds");
        //     }
        // }
        // else {
        //         panic!("Index i out of bounds");
        // }

        if i >= self.all_crabs.len() || j >= self.all_crabs.len() {
            panic!("Index out of bounds");
        }

        let parent_i = &self.all_crabs[i];
        let parent_j = &self.all_crabs[j];

        let childs_color = Color::cross(parent_i.color(), parent_j.color());
        let childs_diet = Diet::random_diet();
        let child_born = Crab::new(name, 1, childs_color, childs_diet);

        self.add_crab(child_born);


    }
    





    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        &self.clans
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        let clan = &mut self.clans;
        clan.add_member(clan_id, crab_name)
     }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        let clan1 = self.clans.get_clan_member_names(id1);
        let clan2 =self.clans.get_clan_member_names(id2);

        if clan1.is_empty(){
            return Err("ivalid".to_string())
        }
        if clan2.is_empty(){
            return Err("ivalid".to_string())
        }
        let avg1_clan1 =self.cal_avg(&clan1);
        let avg2_clan2 =self.cal_avg(&clan2);

            //Ok and err work together
        if avg1_clan1 > avg2_clan2  {
            Ok(Some(id1.to_string()))
        }
        else if avg2_clan2 > avg1_clan1{
            Ok(Some(id2.to_string()))
        }
        else{
            Ok(None)
        }
    }

    pub fn cal_avg(&self, clan: &[String]) -> f64 {
    let sum: f64 = self.all_crabs
        .iter()
        .filter(|crab| clan.contains(&crab.name().to_string()))
        .map(|crab| crab.speed() as f64)
        .sum();

    sum / clan.len() as f64
}
}
