use crate::color::Color;
use crate::cookbook::{Cookbook, Recipe};
use crate::diet::Diet;
use crate::prey::Prey;
use crate::reef::Reef;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Crab {
    // TODO: Add fields here (some in part 1, some in part 2)
    // comes from the crates we imported
    name : String,
    speed: u32,
    color: Color,
    diet: Diet,
    //adding reefs to this struct
    reefs: Vec<Rc<RefCell<Reef>>>
}



// Do NOT implement Copy for Crab.
impl Crab {
    pub fn new(name: String, speed: u32, color: Color, diet: Diet) -> Crab {
        Crab { name , speed, color, diet, reefs: Vec::new() } //similar structure found in colors and cookbook
    }

    // the "&" allows for us to have access to name
    //without taking ownershi over it. thus we are letting
    //the method borrow the field name
    pub fn name(&self) -> &str {
        &self.name
    }

    //not to sure why for speed and Diet we are allowed to
    //return directly the fields and not the reference only
    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn diet(&self) -> Diet {
        self.diet
    }

    // PART 2 BELOW
    // ------------

    /**
     * Have this crab discover a new reef, adding it to its list of reefs.
     */
    pub fn discover_reef(&mut self, reef: Rc<RefCell<Reef>>) {
        self.reefs.push(reef)
    }

    /**
     * Returns Some prey from one of the reefs this crab feeds from,
     * and the index of that reef in self.reefs if able to find Some prey
     * using the `take_prey` method of Reef.
     *
     * If `take_prey` returns None, try the next reef. Try each reef only once.
     *
     * If all reefs are empty, or this crab has no reefs, return None.
     */
    fn catch_prey(&mut self) -> Option<(Box<dyn Prey>, usize)> {
        for (index, reef) in self.reefs.iter().enumerate(){
            let mut reef = reef.borrow_mut();
            if let Some(prey) = reef.take_prey(){
                return Some((prey,index));
            }
        }
        return None
    }

    /**
     * Releases the given prey back into the reef at the given index.
     */
    fn release_prey(&mut self, prey: Box<dyn Prey>, reef_index: usize) {
        //if let Some(reef) = self.reefs.get_mut(reef_index){
        if let Some(reef) = self.reefs.get(reef_index){
            let mut reef = reef.borrow_mut();
            reef.add_prey(prey);
        }
    }

    /**
     * Have this crab go hunting.
     *
     * A crab will keep trying to catch prey until it succeeds,
     * or runs out of remaining prey to try to catch.
     *
     * You should keep track of all escaped prey in a local.
     *
     * Once you have finished hunting, release all escaped prey back into
     * the reefs from whence they came _before_ you return if prey was caught.
     *
     * Your algorithm might look something like this pseudocode. The challenge
     * in this task is not intended to be in figuring out the algorithm, but
     * in figuring out _how to write it in Rust_ using what you've learned so far.
     *
     * ```text
     *     there are no escaped prey yet
     *     prey has not been caught
     *     while prey can be caught
     *       if prey escapes
     *         mark the prey as escaped
     *         try again
     *     
     *       if prey is not edible by this crab
     *         mark the prey as escaped
     *         try again
     *       
     *       prey has been caught
     *       stop trying
     *     
     *     release each escaped prey back to its reef
     *     was prey caught?
     * ```
     *
     * Note: this pseudocode reads like a terrible poem.
     */
    pub fn hunt(&mut self) -> bool {
        let mut escaped = Vec::new();
        let mut catched = false;

        //while prey can be caught
        //loop{
        while let Some((mut prey, index)) = self.catch_prey() {
            //if let Some((mut prey, index)) = self.catch_prey(){
            if prey.try_escape(self){
                escaped.push((prey, index));
            }  
            else if prey.diet() == self.diet() {
                catched = true;
                break;
            }      
            else{
                escaped.push((prey,index));
            }


            if catched {break;}
        }

            

        //release escaped
        for (prey, index) in escaped {
            self.release_prey(prey, index);
        }
        return catched
    }


    /**
     * Returns Some of any recipe from the given cookbook that matches the crab's diet
     * preferences, or None if no such recipe exists.
     *
     * IMPORTANT: you will need to add lifetime parameters to this function. It is
     * up to you to figure out which ones and where. Do not make any other changes
     * to the signature.
     */
    pub fn choose_recipe<'a>(&self, cookbook: &'a Cookbook) -> Option<&'a Recipe> {
        for ind_recipe in cookbook.recipes(){
            if ind_recipe.diet() == self.diet(){
                return Some(ind_recipe);
            }
        }
        return None
    }
}
