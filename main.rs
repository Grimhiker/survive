/* How to survive
 * 5 Cs for survival cordage, cutting tool, cover, container, combustion 
 * locate a water source  
 *create fire, interchangable with water & shelter dependant on situation 
 *create shelter  
 *make water pottable 
 *once fire water and shelter are taken care of go find food 
 
 */
pub struct State {
   cordage: Option<Cordage>,
   cutting_tool: Option<CuttingTool>,
   cover: Option<Cover>,
   container: Option<Container>,
   combustion: Option<Combustion>,
}
 fn Survive() {
    let mut state = State {
        cordage: None,
        cutting_tool: None,
        cover: None,
        container: None,
        combustion: None,
    };
 
   
