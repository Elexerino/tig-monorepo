use super::{Job, NonceIterator};
use crate::future_utils;
use future_utils::{spawn, time, yield_now, Mutex};
use std::sync::Arc;
use tig_challenges::ChallengeTrait;
use tig_worker::{compute_solution, ComputeResult, SolutionData};

pub async fn execute(
    nonce_iters: Vec<Arc<Mutex<NonceIterator>>>,
    job: &Job,
    wasm: &Vec<u8>,
    solutions_data: Arc<Mutex<Vec<SolutionData>>>,
    solutions_count: Arc<Mutex<u32>>,
) {
    for nonce_iter in nonce_iters {
        let job = job.clone();
        let wasm = wasm.clone();
        let solutions_data = solutions_data.clone();
        let solutions_count = solutions_count.clone();
        spawn(async move {
            let mut last_yield = time();
            loop {
                match {
                    let mut nonce_iter = (*nonce_iter).lock().await;
                    (*nonce_iter).next()
                } {
                    None => break,
                    Some(nonce) => {
                        let now = time();
                        if now - last_yield > 25 {
                            yield_now().await;
                            last_yield = now;
                        }
                        let mut run_wasm = true;
                        match job.settings.challenge_id.as_str() {
                            "c001" => {
                                type SolveChallengeFn = fn(&tig_challenges::satisfiability::Challenge) -> anyhow::Result<Option<tig_challenges::satisfiability::Solution>>;
                                if let Some(solve_challenge) =
                                    match job.settings.algorithm_id.as_str() {
                                        // #[cfg(feature = "c001_a001")]
                                        // "c001_a001" => Some(tig_algorithms::satisfiability::c001_a001::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a002")]
                                        // "c001_a002" => Some(tig_algorithms::satisfiability::c001_a002::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a003")]
                                        // "c001_a003" => Some(tig_algorithms::satisfiability::c001_a003::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a004")]
                                        // "c001_a004" => Some(tig_algorithms::satisfiability::c001_a004::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a005")]
                                        // "c001_a005" => Some(tig_algorithms::satisfiability::c001_a005::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a006")]
                                        // "c001_a006" => Some(tig_algorithms::satisfiability::c001_a006::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a007")]
                                        // "c001_a007" => Some(tig_algorithms::satisfiability::c001_a007::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a008")]
                                        // "c001_a008" => Some(tig_algorithms::satisfiability::c001_a008::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a009")]
                                        // "c001_a009" => Some(tig_algorithms::satisfiability::c001_a009::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a010")]
                                        // "c001_a010" => Some(tig_algorithms::satisfiability::c001_a010::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a011")]
                                        // "c001_a011" => Some(tig_algorithms::satisfiability::c001_a011::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a012")]
                                        // "c001_a012" => Some(tig_algorithms::satisfiability::c001_a012::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a013")]
                                        // "c001_a013" => Some(tig_algorithms::satisfiability::c001_a013::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a014")]
                                        // "c001_a014" => Some(tig_algorithms::satisfiability::c001_a014::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a015")]
                                        // "c001_a015" => Some(tig_algorithms::satisfiability::c001_a015::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a016")]
                                        // "c001_a016" => Some(tig_algorithms::satisfiability::c001_a016::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a017")]
                                        // "c001_a017" => Some(tig_algorithms::satisfiability::c001_a017::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a018")]
                                        // "c001_a018" => Some(tig_algorithms::satisfiability::c001_a018::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a019")]
                                        // "c001_a019" => Some(tig_algorithms::satisfiability::c001_a019::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a020")]
                                        // "c001_a020" => Some(tig_algorithms::satisfiability::c001_a020::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a021")]
                                        // "c001_a021" => Some(tig_algorithms::satisfiability::c001_a021::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a022")]
                                        // "c001_a022" => Some(tig_algorithms::satisfiability::c001_a022::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a023")]
                                        // "c001_a023" => Some(tig_algorithms::satisfiability::c001_a023::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a024")]
                                        // "c001_a024" => Some(tig_algorithms::satisfiability::c001_a024::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a025")]
                                        // "c001_a025" => Some(tig_algorithms::satisfiability::c001_a025::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a026")]
                                        // "c001_a026" => Some(tig_algorithms::satisfiability::c001_a026::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a027")]
                                        // "c001_a027" => Some(tig_algorithms::satisfiability::c001_a027::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a028")]
                                        // "c001_a028" => Some(tig_algorithms::satisfiability::c001_a028::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a029")]
                                        // "c001_a029" => Some(tig_algorithms::satisfiability::c001_a029::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a030")]
                                        // "c001_a030" => Some(tig_algorithms::satisfiability::c001_a030::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a031")]
                                        // "c001_a031" => Some(tig_algorithms::satisfiability::c001_a031::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a032")]
                                        // "c001_a032" => Some(tig_algorithms::satisfiability::c001_a032::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a033")]
                                        // "c001_a033" => Some(tig_algorithms::satisfiability::c001_a033::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a034")]
                                        // "c001_a034" => Some(tig_algorithms::satisfiability::c001_a034::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a035")]
                                        // "c001_a035" => Some(tig_algorithms::satisfiability::c001_a035::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a036")]
                                        // "c001_a036" => Some(tig_algorithms::satisfiability::c001_a036::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a037")]
                                        // "c001_a037" => Some(tig_algorithms::satisfiability::c001_a037::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a038")]
                                        // "c001_a038" => Some(tig_algorithms::satisfiability::c001_a038::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a039")]
                                        // "c001_a039" => Some(tig_algorithms::satisfiability::c001_a039::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a040")]
                                        // "c001_a040" => Some(tig_algorithms::satisfiability::c001_a040::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a041")]
                                        // "c001_a041" => Some(tig_algorithms::satisfiability::c001_a041::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a042")]
                                        // "c001_a042" => Some(tig_algorithms::satisfiability::c001_a042::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a043")]
                                        // "c001_a043" => Some(tig_algorithms::satisfiability::c001_a043::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a044")]
                                        // "c001_a044" => Some(tig_algorithms::satisfiability::c001_a044::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a045")]
                                        // "c001_a045" => Some(tig_algorithms::satisfiability::c001_a045::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a046")]
                                        // "c001_a046" => Some(tig_algorithms::satisfiability::c001_a046::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a047")]
                                        // "c001_a047" => Some(tig_algorithms::satisfiability::c001_a047::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a048")]
                                        // "c001_a048" => Some(tig_algorithms::satisfiability::c001_a048::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a049")]
                                        // "c001_a049" => Some(tig_algorithms::satisfiability::c001_a049::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a050")]
                                        // "c001_a050" => Some(tig_algorithms::satisfiability::c001_a050::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a051")]
                                        // "c001_a051" => Some(tig_algorithms::satisfiability::c001_a051::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a052")]
                                        // "c001_a052" => Some(tig_algorithms::satisfiability::c001_a052::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a053")]
                                        // "c001_a053" => Some(tig_algorithms::satisfiability::c001_a053::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a054")]
                                        // "c001_a054" => Some(tig_algorithms::satisfiability::c001_a054::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a055")]
                                        // "c001_a055" => Some(tig_algorithms::satisfiability::c001_a055::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a056")]
                                        // "c001_a056" => Some(tig_algorithms::satisfiability::c001_a056::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a057")]
                                        // "c001_a057" => Some(tig_algorithms::satisfiability::c001_a057::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a058")]
                                        // "c001_a058" => Some(tig_algorithms::satisfiability::c001_a058::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a059")]
                                        // "c001_a059" => Some(tig_algorithms::satisfiability::c001_a059::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a060")]
                                        // "c001_a060" => Some(tig_algorithms::satisfiability::c001_a060::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a061")]
                                        // "c001_a061" => Some(tig_algorithms::satisfiability::c001_a061::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a062")]
                                        // "c001_a062" => Some(tig_algorithms::satisfiability::c001_a062::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a063")]
                                        // "c001_a063" => Some(tig_algorithms::satisfiability::c001_a063::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a064")]
                                        // "c001_a064" => Some(tig_algorithms::satisfiability::c001_a064::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a065")]
                                        // "c001_a065" => Some(tig_algorithms::satisfiability::c001_a065::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a066")]
                                        // "c001_a066" => Some(tig_algorithms::satisfiability::c001_a066::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a067")]
                                        // "c001_a067" => Some(tig_algorithms::satisfiability::c001_a067::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a068")]
                                        // "c001_a068" => Some(tig_algorithms::satisfiability::c001_a068::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a069")]
                                        // "c001_a069" => Some(tig_algorithms::satisfiability::c001_a069::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a070")]
                                        // "c001_a070" => Some(tig_algorithms::satisfiability::c001_a070::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a071")]
                                        // "c001_a071" => Some(tig_algorithms::satisfiability::c001_a071::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a072")]
                                        // "c001_a072" => Some(tig_algorithms::satisfiability::c001_a072::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a073")]
                                        // "c001_a073" => Some(tig_algorithms::satisfiability::c001_a073::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a074")]
                                        // "c001_a074" => Some(tig_algorithms::satisfiability::c001_a074::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a075")]
                                        // "c001_a075" => Some(tig_algorithms::satisfiability::c001_a075::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a076")]
                                        // "c001_a076" => Some(tig_algorithms::satisfiability::c001_a076::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a077")]
                                        // "c001_a077" => Some(tig_algorithms::satisfiability::c001_a077::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a078")]
                                        // "c001_a078" => Some(tig_algorithms::satisfiability::c001_a078::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a079")]
                                        // "c001_a079" => Some(tig_algorithms::satisfiability::c001_a079::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a080")]
                                        // "c001_a080" => Some(tig_algorithms::satisfiability::c001_a080::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a081")]
                                        // "c001_a081" => Some(tig_algorithms::satisfiability::c001_a081::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a082")]
                                        // "c001_a082" => Some(tig_algorithms::satisfiability::c001_a082::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a083")]
                                        // "c001_a083" => Some(tig_algorithms::satisfiability::c001_a083::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a084")]
                                        // "c001_a084" => Some(tig_algorithms::satisfiability::c001_a084::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a085")]
                                        // "c001_a085" => Some(tig_algorithms::satisfiability::c001_a085::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a086")]
                                        // "c001_a086" => Some(tig_algorithms::satisfiability::c001_a086::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a087")]
                                        // "c001_a087" => Some(tig_algorithms::satisfiability::c001_a087::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a088")]
                                        // "c001_a088" => Some(tig_algorithms::satisfiability::c001_a088::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a089")]
                                        // "c001_a089" => Some(tig_algorithms::satisfiability::c001_a089::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a090")]
                                        // "c001_a090" => Some(tig_algorithms::satisfiability::c001_a090::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a091")]
                                        // "c001_a091" => Some(tig_algorithms::satisfiability::c001_a091::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a092")]
                                        // "c001_a092" => Some(tig_algorithms::satisfiability::c001_a092::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a093")]
                                        // "c001_a093" => Some(tig_algorithms::satisfiability::c001_a093::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a094")]
                                        // "c001_a094" => Some(tig_algorithms::satisfiability::c001_a094::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a095")]
                                        // "c001_a095" => Some(tig_algorithms::satisfiability::c001_a095::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a096")]
                                        // "c001_a096" => Some(tig_algorithms::satisfiability::c001_a096::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a097")]
                                        // "c001_a097" => Some(tig_algorithms::satisfiability::c001_a097::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a098")]
                                        // "c001_a098" => Some(tig_algorithms::satisfiability::c001_a098::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a099")]
                                        // "c001_a099" => Some(tig_algorithms::satisfiability::c001_a099::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a100")]
                                        // "c001_a100" => Some(tig_algorithms::satisfiability::c001_a100::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a101")]
                                        // "c001_a101" => Some(tig_algorithms::satisfiability::c001_a101::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a102")]
                                        // "c001_a102" => Some(tig_algorithms::satisfiability::c001_a102::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a103")]
                                        // "c001_a103" => Some(tig_algorithms::satisfiability::c001_a103::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a104")]
                                        // "c001_a104" => Some(tig_algorithms::satisfiability::c001_a104::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a105")]
                                        // "c001_a105" => Some(tig_algorithms::satisfiability::c001_a105::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a106")]
                                        // "c001_a106" => Some(tig_algorithms::satisfiability::c001_a106::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a107")]
                                        // "c001_a107" => Some(tig_algorithms::satisfiability::c001_a107::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a108")]
                                        // "c001_a108" => Some(tig_algorithms::satisfiability::c001_a108::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a109")]
                                        // "c001_a109" => Some(tig_algorithms::satisfiability::c001_a109::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a110")]
                                        // "c001_a110" => Some(tig_algorithms::satisfiability::c001_a110::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a111")]
                                        // "c001_a111" => Some(tig_algorithms::satisfiability::c001_a111::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a112")]
                                        // "c001_a112" => Some(tig_algorithms::satisfiability::c001_a112::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a113")]
                                        // "c001_a113" => Some(tig_algorithms::satisfiability::c001_a113::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a114")]
                                        // "c001_a114" => Some(tig_algorithms::satisfiability::c001_a114::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a115")]
                                        // "c001_a115" => Some(tig_algorithms::satisfiability::c001_a115::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a116")]
                                        // "c001_a116" => Some(tig_algorithms::satisfiability::c001_a116::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a117")]
                                        // "c001_a117" => Some(tig_algorithms::satisfiability::c001_a117::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a118")]
                                        // "c001_a118" => Some(tig_algorithms::satisfiability::c001_a118::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a119")]
                                        // "c001_a119" => Some(tig_algorithms::satisfiability::c001_a119::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a120")]
                                        // "c001_a120" => Some(tig_algorithms::satisfiability::c001_a120::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a121")]
                                        // "c001_a121" => Some(tig_algorithms::satisfiability::c001_a121::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a122")]
                                        // "c001_a122" => Some(tig_algorithms::satisfiability::c001_a122::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a123")]
                                        // "c001_a123" => Some(tig_algorithms::satisfiability::c001_a123::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a124")]
                                        // "c001_a124" => Some(tig_algorithms::satisfiability::c001_a124::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a125")]
                                        // "c001_a125" => Some(tig_algorithms::satisfiability::c001_a125::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a126")]
                                        // "c001_a126" => Some(tig_algorithms::satisfiability::c001_a126::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a127")]
                                        // "c001_a127" => Some(tig_algorithms::satisfiability::c001_a127::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a128")]
                                        // "c001_a128" => Some(tig_algorithms::satisfiability::c001_a128::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a129")]
                                        // "c001_a129" => Some(tig_algorithms::satisfiability::c001_a129::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a130")]
                                        // "c001_a130" => Some(tig_algorithms::satisfiability::c001_a130::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a131")]
                                        // "c001_a131" => Some(tig_algorithms::satisfiability::c001_a131::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a132")]
                                        // "c001_a132" => Some(tig_algorithms::satisfiability::c001_a132::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a133")]
                                        // "c001_a133" => Some(tig_algorithms::satisfiability::c001_a133::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a134")]
                                        // "c001_a134" => Some(tig_algorithms::satisfiability::c001_a134::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a135")]
                                        // "c001_a135" => Some(tig_algorithms::satisfiability::c001_a135::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a136")]
                                        // "c001_a136" => Some(tig_algorithms::satisfiability::c001_a136::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a137")]
                                        // "c001_a137" => Some(tig_algorithms::satisfiability::c001_a137::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a138")]
                                        // "c001_a138" => Some(tig_algorithms::satisfiability::c001_a138::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a139")]
                                        // "c001_a139" => Some(tig_algorithms::satisfiability::c001_a139::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a140")]
                                        // "c001_a140" => Some(tig_algorithms::satisfiability::c001_a140::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a141")]
                                        // "c001_a141" => Some(tig_algorithms::satisfiability::c001_a141::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a142")]
                                        // "c001_a142" => Some(tig_algorithms::satisfiability::c001_a142::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a143")]
                                        // "c001_a143" => Some(tig_algorithms::satisfiability::c001_a143::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a144")]
                                        // "c001_a144" => Some(tig_algorithms::satisfiability::c001_a144::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a145")]
                                        // "c001_a145" => Some(tig_algorithms::satisfiability::c001_a145::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a146")]
                                        // "c001_a146" => Some(tig_algorithms::satisfiability::c001_a146::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a147")]
                                        // "c001_a147" => Some(tig_algorithms::satisfiability::c001_a147::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a148")]
                                        // "c001_a148" => Some(tig_algorithms::satisfiability::c001_a148::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a149")]
                                        // "c001_a149" => Some(tig_algorithms::satisfiability::c001_a149::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a150")]
                                        // "c001_a150" => Some(tig_algorithms::satisfiability::c001_a150::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a151")]
                                        // "c001_a151" => Some(tig_algorithms::satisfiability::c001_a151::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a152")]
                                        // "c001_a152" => Some(tig_algorithms::satisfiability::c001_a152::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a153")]
                                        // "c001_a153" => Some(tig_algorithms::satisfiability::c001_a153::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a154")]
                                        // "c001_a154" => Some(tig_algorithms::satisfiability::c001_a154::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a155")]
                                        // "c001_a155" => Some(tig_algorithms::satisfiability::c001_a155::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a156")]
                                        // "c001_a156" => Some(tig_algorithms::satisfiability::c001_a156::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a157")]
                                        // "c001_a157" => Some(tig_algorithms::satisfiability::c001_a157::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a158")]
                                        // "c001_a158" => Some(tig_algorithms::satisfiability::c001_a158::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a159")]
                                        // "c001_a159" => Some(tig_algorithms::satisfiability::c001_a159::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a160")]
                                        // "c001_a160" => Some(tig_algorithms::satisfiability::c001_a160::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a161")]
                                        // "c001_a161" => Some(tig_algorithms::satisfiability::c001_a161::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a162")]
                                        // "c001_a162" => Some(tig_algorithms::satisfiability::c001_a162::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a163")]
                                        // "c001_a163" => Some(tig_algorithms::satisfiability::c001_a163::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a164")]
                                        // "c001_a164" => Some(tig_algorithms::satisfiability::c001_a164::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a165")]
                                        // "c001_a165" => Some(tig_algorithms::satisfiability::c001_a165::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a166")]
                                        // "c001_a166" => Some(tig_algorithms::satisfiability::c001_a166::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a167")]
                                        // "c001_a167" => Some(tig_algorithms::satisfiability::c001_a167::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a168")]
                                        // "c001_a168" => Some(tig_algorithms::satisfiability::c001_a168::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a169")]
                                        // "c001_a169" => Some(tig_algorithms::satisfiability::c001_a169::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a170")]
                                        // "c001_a170" => Some(tig_algorithms::satisfiability::c001_a170::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a171")]
                                        // "c001_a171" => Some(tig_algorithms::satisfiability::c001_a171::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a172")]
                                        // "c001_a172" => Some(tig_algorithms::satisfiability::c001_a172::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a173")]
                                        // "c001_a173" => Some(tig_algorithms::satisfiability::c001_a173::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a174")]
                                        // "c001_a174" => Some(tig_algorithms::satisfiability::c001_a174::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a175")]
                                        // "c001_a175" => Some(tig_algorithms::satisfiability::c001_a175::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a176")]
                                        // "c001_a176" => Some(tig_algorithms::satisfiability::c001_a176::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a177")]
                                        // "c001_a177" => Some(tig_algorithms::satisfiability::c001_a177::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a178")]
                                        // "c001_a178" => Some(tig_algorithms::satisfiability::c001_a178::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a179")]
                                        // "c001_a179" => Some(tig_algorithms::satisfiability::c001_a179::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a180")]
                                        // "c001_a180" => Some(tig_algorithms::satisfiability::c001_a180::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a181")]
                                        // "c001_a181" => Some(tig_algorithms::satisfiability::c001_a181::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a182")]
                                        // "c001_a182" => Some(tig_algorithms::satisfiability::c001_a182::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a183")]
                                        // "c001_a183" => Some(tig_algorithms::satisfiability::c001_a183::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a184")]
                                        // "c001_a184" => Some(tig_algorithms::satisfiability::c001_a184::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a185")]
                                        // "c001_a185" => Some(tig_algorithms::satisfiability::c001_a185::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a186")]
                                        // "c001_a186" => Some(tig_algorithms::satisfiability::c001_a186::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a187")]
                                        // "c001_a187" => Some(tig_algorithms::satisfiability::c001_a187::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a188")]
                                        // "c001_a188" => Some(tig_algorithms::satisfiability::c001_a188::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a189")]
                                        // "c001_a189" => Some(tig_algorithms::satisfiability::c001_a189::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a190")]
                                        // "c001_a190" => Some(tig_algorithms::satisfiability::c001_a190::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a191")]
                                        // "c001_a191" => Some(tig_algorithms::satisfiability::c001_a191::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a192")]
                                        // "c001_a192" => Some(tig_algorithms::satisfiability::c001_a192::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a193")]
                                        // "c001_a193" => Some(tig_algorithms::satisfiability::c001_a193::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a194")]
                                        // "c001_a194" => Some(tig_algorithms::satisfiability::c001_a194::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a195")]
                                        // "c001_a195" => Some(tig_algorithms::satisfiability::c001_a195::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a196")]
                                        // "c001_a196" => Some(tig_algorithms::satisfiability::c001_a196::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a197")]
                                        // "c001_a197" => Some(tig_algorithms::satisfiability::c001_a197::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a198")]
                                        // "c001_a198" => Some(tig_algorithms::satisfiability::c001_a198::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a199")]
                                        // "c001_a199" => Some(tig_algorithms::satisfiability::c001_a199::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a200")]
                                        // "c001_a200" => Some(tig_algorithms::satisfiability::c001_a200::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a201")]
                                        // "c001_a201" => Some(tig_algorithms::satisfiability::c001_a201::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a202")]
                                        // "c001_a202" => Some(tig_algorithms::satisfiability::c001_a202::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a203")]
                                        // "c001_a203" => Some(tig_algorithms::satisfiability::c001_a203::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a204")]
                                        // "c001_a204" => Some(tig_algorithms::satisfiability::c001_a204::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a205")]
                                        // "c001_a205" => Some(tig_algorithms::satisfiability::c001_a205::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a206")]
                                        // "c001_a206" => Some(tig_algorithms::satisfiability::c001_a206::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a207")]
                                        // "c001_a207" => Some(tig_algorithms::satisfiability::c001_a207::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a208")]
                                        // "c001_a208" => Some(tig_algorithms::satisfiability::c001_a208::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a209")]
                                        // "c001_a209" => Some(tig_algorithms::satisfiability::c001_a209::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a210")]
                                        // "c001_a210" => Some(tig_algorithms::satisfiability::c001_a210::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a211")]
                                        // "c001_a211" => Some(tig_algorithms::satisfiability::c001_a211::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a212")]
                                        // "c001_a212" => Some(tig_algorithms::satisfiability::c001_a212::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a213")]
                                        // "c001_a213" => Some(tig_algorithms::satisfiability::c001_a213::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a214")]
                                        // "c001_a214" => Some(tig_algorithms::satisfiability::c001_a214::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a215")]
                                        // "c001_a215" => Some(tig_algorithms::satisfiability::c001_a215::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a216")]
                                        // "c001_a216" => Some(tig_algorithms::satisfiability::c001_a216::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a217")]
                                        // "c001_a217" => Some(tig_algorithms::satisfiability::c001_a217::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a218")]
                                        // "c001_a218" => Some(tig_algorithms::satisfiability::c001_a218::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a219")]
                                        // "c001_a219" => Some(tig_algorithms::satisfiability::c001_a219::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a220")]
                                        // "c001_a220" => Some(tig_algorithms::satisfiability::c001_a220::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a221")]
                                        // "c001_a221" => Some(tig_algorithms::satisfiability::c001_a221::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a222")]
                                        // "c001_a222" => Some(tig_algorithms::satisfiability::c001_a222::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a223")]
                                        // "c001_a223" => Some(tig_algorithms::satisfiability::c001_a223::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a224")]
                                        // "c001_a224" => Some(tig_algorithms::satisfiability::c001_a224::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a225")]
                                        // "c001_a225" => Some(tig_algorithms::satisfiability::c001_a225::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a226")]
                                        // "c001_a226" => Some(tig_algorithms::satisfiability::c001_a226::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a227")]
                                        // "c001_a227" => Some(tig_algorithms::satisfiability::c001_a227::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a228")]
                                        // "c001_a228" => Some(tig_algorithms::satisfiability::c001_a228::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a229")]
                                        // "c001_a229" => Some(tig_algorithms::satisfiability::c001_a229::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a230")]
                                        // "c001_a230" => Some(tig_algorithms::satisfiability::c001_a230::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a231")]
                                        // "c001_a231" => Some(tig_algorithms::satisfiability::c001_a231::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a232")]
                                        // "c001_a232" => Some(tig_algorithms::satisfiability::c001_a232::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a233")]
                                        // "c001_a233" => Some(tig_algorithms::satisfiability::c001_a233::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a234")]
                                        // "c001_a234" => Some(tig_algorithms::satisfiability::c001_a234::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a235")]
                                        // "c001_a235" => Some(tig_algorithms::satisfiability::c001_a235::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a236")]
                                        // "c001_a236" => Some(tig_algorithms::satisfiability::c001_a236::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a237")]
                                        // "c001_a237" => Some(tig_algorithms::satisfiability::c001_a237::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a238")]
                                        // "c001_a238" => Some(tig_algorithms::satisfiability::c001_a238::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a239")]
                                        // "c001_a239" => Some(tig_algorithms::satisfiability::c001_a239::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a240")]
                                        // "c001_a240" => Some(tig_algorithms::satisfiability::c001_a240::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a241")]
                                        // "c001_a241" => Some(tig_algorithms::satisfiability::c001_a241::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a242")]
                                        // "c001_a242" => Some(tig_algorithms::satisfiability::c001_a242::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a243")]
                                        // "c001_a243" => Some(tig_algorithms::satisfiability::c001_a243::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a244")]
                                        // "c001_a244" => Some(tig_algorithms::satisfiability::c001_a244::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a245")]
                                        // "c001_a245" => Some(tig_algorithms::satisfiability::c001_a245::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a246")]
                                        // "c001_a246" => Some(tig_algorithms::satisfiability::c001_a246::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a247")]
                                        // "c001_a247" => Some(tig_algorithms::satisfiability::c001_a247::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a248")]
                                        // "c001_a248" => Some(tig_algorithms::satisfiability::c001_a248::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a249")]
                                        // "c001_a249" => Some(tig_algorithms::satisfiability::c001_a249::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a250")]
                                        // "c001_a250" => Some(tig_algorithms::satisfiability::c001_a250::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a251")]
                                        // "c001_a251" => Some(tig_algorithms::satisfiability::c001_a251::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a252")]
                                        // "c001_a252" => Some(tig_algorithms::satisfiability::c001_a252::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a253")]
                                        // "c001_a253" => Some(tig_algorithms::satisfiability::c001_a253::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a254")]
                                        // "c001_a254" => Some(tig_algorithms::satisfiability::c001_a254::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a255")]
                                        // "c001_a255" => Some(tig_algorithms::satisfiability::c001_a255::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a256")]
                                        // "c001_a256" => Some(tig_algorithms::satisfiability::c001_a256::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a257")]
                                        // "c001_a257" => Some(tig_algorithms::satisfiability::c001_a257::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a258")]
                                        // "c001_a258" => Some(tig_algorithms::satisfiability::c001_a258::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a259")]
                                        // "c001_a259" => Some(tig_algorithms::satisfiability::c001_a259::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a260")]
                                        // "c001_a260" => Some(tig_algorithms::satisfiability::c001_a260::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a261")]
                                        // "c001_a261" => Some(tig_algorithms::satisfiability::c001_a261::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a262")]
                                        // "c001_a262" => Some(tig_algorithms::satisfiability::c001_a262::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a263")]
                                        // "c001_a263" => Some(tig_algorithms::satisfiability::c001_a263::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a264")]
                                        // "c001_a264" => Some(tig_algorithms::satisfiability::c001_a264::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a265")]
                                        // "c001_a265" => Some(tig_algorithms::satisfiability::c001_a265::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a266")]
                                        // "c001_a266" => Some(tig_algorithms::satisfiability::c001_a266::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a267")]
                                        // "c001_a267" => Some(tig_algorithms::satisfiability::c001_a267::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a268")]
                                        // "c001_a268" => Some(tig_algorithms::satisfiability::c001_a268::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a269")]
                                        // "c001_a269" => Some(tig_algorithms::satisfiability::c001_a269::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a270")]
                                        // "c001_a270" => Some(tig_algorithms::satisfiability::c001_a270::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a271")]
                                        // "c001_a271" => Some(tig_algorithms::satisfiability::c001_a271::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a272")]
                                        // "c001_a272" => Some(tig_algorithms::satisfiability::c001_a272::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a273")]
                                        // "c001_a273" => Some(tig_algorithms::satisfiability::c001_a273::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a274")]
                                        // "c001_a274" => Some(tig_algorithms::satisfiability::c001_a274::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a275")]
                                        // "c001_a275" => Some(tig_algorithms::satisfiability::c001_a275::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a276")]
                                        // "c001_a276" => Some(tig_algorithms::satisfiability::c001_a276::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a277")]
                                        // "c001_a277" => Some(tig_algorithms::satisfiability::c001_a277::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a278")]
                                        // "c001_a278" => Some(tig_algorithms::satisfiability::c001_a278::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a279")]
                                        // "c001_a279" => Some(tig_algorithms::satisfiability::c001_a279::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a280")]
                                        // "c001_a280" => Some(tig_algorithms::satisfiability::c001_a280::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a281")]
                                        // "c001_a281" => Some(tig_algorithms::satisfiability::c001_a281::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a282")]
                                        // "c001_a282" => Some(tig_algorithms::satisfiability::c001_a282::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a283")]
                                        // "c001_a283" => Some(tig_algorithms::satisfiability::c001_a283::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a284")]
                                        // "c001_a284" => Some(tig_algorithms::satisfiability::c001_a284::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a285")]
                                        // "c001_a285" => Some(tig_algorithms::satisfiability::c001_a285::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a286")]
                                        // "c001_a286" => Some(tig_algorithms::satisfiability::c001_a286::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a287")]
                                        // "c001_a287" => Some(tig_algorithms::satisfiability::c001_a287::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a288")]
                                        // "c001_a288" => Some(tig_algorithms::satisfiability::c001_a288::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a289")]
                                        // "c001_a289" => Some(tig_algorithms::satisfiability::c001_a289::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a290")]
                                        // "c001_a290" => Some(tig_algorithms::satisfiability::c001_a290::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a291")]
                                        // "c001_a291" => Some(tig_algorithms::satisfiability::c001_a291::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a292")]
                                        // "c001_a292" => Some(tig_algorithms::satisfiability::c001_a292::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a293")]
                                        // "c001_a293" => Some(tig_algorithms::satisfiability::c001_a293::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a294")]
                                        // "c001_a294" => Some(tig_algorithms::satisfiability::c001_a294::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a295")]
                                        // "c001_a295" => Some(tig_algorithms::satisfiability::c001_a295::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a296")]
                                        // "c001_a296" => Some(tig_algorithms::satisfiability::c001_a296::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a297")]
                                        // "c001_a297" => Some(tig_algorithms::satisfiability::c001_a297::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a298")]
                                        // "c001_a298" => Some(tig_algorithms::satisfiability::c001_a298::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a299")]
                                        // "c001_a299" => Some(tig_algorithms::satisfiability::c001_a299::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a300")]
                                        // "c001_a300" => Some(tig_algorithms::satisfiability::c001_a300::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a301")]
                                        // "c001_a301" => Some(tig_algorithms::satisfiability::c001_a301::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a302")]
                                        // "c001_a302" => Some(tig_algorithms::satisfiability::c001_a302::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a303")]
                                        // "c001_a303" => Some(tig_algorithms::satisfiability::c001_a303::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a304")]
                                        // "c001_a304" => Some(tig_algorithms::satisfiability::c001_a304::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a305")]
                                        // "c001_a305" => Some(tig_algorithms::satisfiability::c001_a305::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a306")]
                                        // "c001_a306" => Some(tig_algorithms::satisfiability::c001_a306::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a307")]
                                        // "c001_a307" => Some(tig_algorithms::satisfiability::c001_a307::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a308")]
                                        // "c001_a308" => Some(tig_algorithms::satisfiability::c001_a308::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a309")]
                                        // "c001_a309" => Some(tig_algorithms::satisfiability::c001_a309::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a310")]
                                        // "c001_a310" => Some(tig_algorithms::satisfiability::c001_a310::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a311")]
                                        // "c001_a311" => Some(tig_algorithms::satisfiability::c001_a311::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a312")]
                                        // "c001_a312" => Some(tig_algorithms::satisfiability::c001_a312::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a313")]
                                        // "c001_a313" => Some(tig_algorithms::satisfiability::c001_a313::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a314")]
                                        // "c001_a314" => Some(tig_algorithms::satisfiability::c001_a314::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a315")]
                                        // "c001_a315" => Some(tig_algorithms::satisfiability::c001_a315::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a316")]
                                        // "c001_a316" => Some(tig_algorithms::satisfiability::c001_a316::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a317")]
                                        // "c001_a317" => Some(tig_algorithms::satisfiability::c001_a317::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a318")]
                                        // "c001_a318" => Some(tig_algorithms::satisfiability::c001_a318::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a319")]
                                        // "c001_a319" => Some(tig_algorithms::satisfiability::c001_a319::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a320")]
                                        // "c001_a320" => Some(tig_algorithms::satisfiability::c001_a320::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a321")]
                                        // "c001_a321" => Some(tig_algorithms::satisfiability::c001_a321::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a322")]
                                        // "c001_a322" => Some(tig_algorithms::satisfiability::c001_a322::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a323")]
                                        // "c001_a323" => Some(tig_algorithms::satisfiability::c001_a323::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a324")]
                                        // "c001_a324" => Some(tig_algorithms::satisfiability::c001_a324::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a325")]
                                        // "c001_a325" => Some(tig_algorithms::satisfiability::c001_a325::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a326")]
                                        // "c001_a326" => Some(tig_algorithms::satisfiability::c001_a326::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a327")]
                                        // "c001_a327" => Some(tig_algorithms::satisfiability::c001_a327::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a328")]
                                        // "c001_a328" => Some(tig_algorithms::satisfiability::c001_a328::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a329")]
                                        // "c001_a329" => Some(tig_algorithms::satisfiability::c001_a329::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a330")]
                                        // "c001_a330" => Some(tig_algorithms::satisfiability::c001_a330::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a331")]
                                        // "c001_a331" => Some(tig_algorithms::satisfiability::c001_a331::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a332")]
                                        // "c001_a332" => Some(tig_algorithms::satisfiability::c001_a332::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a333")]
                                        // "c001_a333" => Some(tig_algorithms::satisfiability::c001_a333::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a334")]
                                        // "c001_a334" => Some(tig_algorithms::satisfiability::c001_a334::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a335")]
                                        // "c001_a335" => Some(tig_algorithms::satisfiability::c001_a335::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a336")]
                                        // "c001_a336" => Some(tig_algorithms::satisfiability::c001_a336::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a337")]
                                        // "c001_a337" => Some(tig_algorithms::satisfiability::c001_a337::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a338")]
                                        // "c001_a338" => Some(tig_algorithms::satisfiability::c001_a338::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a339")]
                                        // "c001_a339" => Some(tig_algorithms::satisfiability::c001_a339::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a340")]
                                        // "c001_a340" => Some(tig_algorithms::satisfiability::c001_a340::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a341")]
                                        // "c001_a341" => Some(tig_algorithms::satisfiability::c001_a341::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a342")]
                                        // "c001_a342" => Some(tig_algorithms::satisfiability::c001_a342::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a343")]
                                        // "c001_a343" => Some(tig_algorithms::satisfiability::c001_a343::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a344")]
                                        // "c001_a344" => Some(tig_algorithms::satisfiability::c001_a344::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a345")]
                                        // "c001_a345" => Some(tig_algorithms::satisfiability::c001_a345::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a346")]
                                        // "c001_a346" => Some(tig_algorithms::satisfiability::c001_a346::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a347")]
                                        // "c001_a347" => Some(tig_algorithms::satisfiability::c001_a347::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a348")]
                                        // "c001_a348" => Some(tig_algorithms::satisfiability::c001_a348::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a349")]
                                        // "c001_a349" => Some(tig_algorithms::satisfiability::c001_a349::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a350")]
                                        // "c001_a350" => Some(tig_algorithms::satisfiability::c001_a350::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a351")]
                                        // "c001_a351" => Some(tig_algorithms::satisfiability::c001_a351::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a352")]
                                        // "c001_a352" => Some(tig_algorithms::satisfiability::c001_a352::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a353")]
                                        // "c001_a353" => Some(tig_algorithms::satisfiability::c001_a353::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a354")]
                                        // "c001_a354" => Some(tig_algorithms::satisfiability::c001_a354::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a355")]
                                        // "c001_a355" => Some(tig_algorithms::satisfiability::c001_a355::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a356")]
                                        // "c001_a356" => Some(tig_algorithms::satisfiability::c001_a356::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a357")]
                                        // "c001_a357" => Some(tig_algorithms::satisfiability::c001_a357::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a358")]
                                        // "c001_a358" => Some(tig_algorithms::satisfiability::c001_a358::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a359")]
                                        // "c001_a359" => Some(tig_algorithms::satisfiability::c001_a359::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a360")]
                                        // "c001_a360" => Some(tig_algorithms::satisfiability::c001_a360::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a361")]
                                        // "c001_a361" => Some(tig_algorithms::satisfiability::c001_a361::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a362")]
                                        // "c001_a362" => Some(tig_algorithms::satisfiability::c001_a362::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a363")]
                                        // "c001_a363" => Some(tig_algorithms::satisfiability::c001_a363::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a364")]
                                        // "c001_a364" => Some(tig_algorithms::satisfiability::c001_a364::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a365")]
                                        // "c001_a365" => Some(tig_algorithms::satisfiability::c001_a365::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a366")]
                                        // "c001_a366" => Some(tig_algorithms::satisfiability::c001_a366::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a367")]
                                        // "c001_a367" => Some(tig_algorithms::satisfiability::c001_a367::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a368")]
                                        // "c001_a368" => Some(tig_algorithms::satisfiability::c001_a368::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a369")]
                                        // "c001_a369" => Some(tig_algorithms::satisfiability::c001_a369::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a370")]
                                        // "c001_a370" => Some(tig_algorithms::satisfiability::c001_a370::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a371")]
                                        // "c001_a371" => Some(tig_algorithms::satisfiability::c001_a371::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a372")]
                                        // "c001_a372" => Some(tig_algorithms::satisfiability::c001_a372::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a373")]
                                        // "c001_a373" => Some(tig_algorithms::satisfiability::c001_a373::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a374")]
                                        // "c001_a374" => Some(tig_algorithms::satisfiability::c001_a374::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a375")]
                                        // "c001_a375" => Some(tig_algorithms::satisfiability::c001_a375::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a376")]
                                        // "c001_a376" => Some(tig_algorithms::satisfiability::c001_a376::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a377")]
                                        // "c001_a377" => Some(tig_algorithms::satisfiability::c001_a377::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a378")]
                                        // "c001_a378" => Some(tig_algorithms::satisfiability::c001_a378::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a379")]
                                        // "c001_a379" => Some(tig_algorithms::satisfiability::c001_a379::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a380")]
                                        // "c001_a380" => Some(tig_algorithms::satisfiability::c001_a380::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a381")]
                                        // "c001_a381" => Some(tig_algorithms::satisfiability::c001_a381::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a382")]
                                        // "c001_a382" => Some(tig_algorithms::satisfiability::c001_a382::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a383")]
                                        // "c001_a383" => Some(tig_algorithms::satisfiability::c001_a383::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a384")]
                                        // "c001_a384" => Some(tig_algorithms::satisfiability::c001_a384::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a385")]
                                        // "c001_a385" => Some(tig_algorithms::satisfiability::c001_a385::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a386")]
                                        // "c001_a386" => Some(tig_algorithms::satisfiability::c001_a386::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a387")]
                                        // "c001_a387" => Some(tig_algorithms::satisfiability::c001_a387::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a388")]
                                        // "c001_a388" => Some(tig_algorithms::satisfiability::c001_a388::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a389")]
                                        // "c001_a389" => Some(tig_algorithms::satisfiability::c001_a389::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a390")]
                                        // "c001_a390" => Some(tig_algorithms::satisfiability::c001_a390::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a391")]
                                        // "c001_a391" => Some(tig_algorithms::satisfiability::c001_a391::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a392")]
                                        // "c001_a392" => Some(tig_algorithms::satisfiability::c001_a392::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a393")]
                                        // "c001_a393" => Some(tig_algorithms::satisfiability::c001_a393::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a394")]
                                        // "c001_a394" => Some(tig_algorithms::satisfiability::c001_a394::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a395")]
                                        // "c001_a395" => Some(tig_algorithms::satisfiability::c001_a395::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a396")]
                                        // "c001_a396" => Some(tig_algorithms::satisfiability::c001_a396::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a397")]
                                        // "c001_a397" => Some(tig_algorithms::satisfiability::c001_a397::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a398")]
                                        // "c001_a398" => Some(tig_algorithms::satisfiability::c001_a398::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a399")]
                                        // "c001_a399" => Some(tig_algorithms::satisfiability::c001_a399::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a400")]
                                        // "c001_a400" => Some(tig_algorithms::satisfiability::c001_a400::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a401")]
                                        // "c001_a401" => Some(tig_algorithms::satisfiability::c001_a401::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a402")]
                                        // "c001_a402" => Some(tig_algorithms::satisfiability::c001_a402::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a403")]
                                        // "c001_a403" => Some(tig_algorithms::satisfiability::c001_a403::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a404")]
                                        // "c001_a404" => Some(tig_algorithms::satisfiability::c001_a404::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a405")]
                                        // "c001_a405" => Some(tig_algorithms::satisfiability::c001_a405::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a406")]
                                        // "c001_a406" => Some(tig_algorithms::satisfiability::c001_a406::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a407")]
                                        // "c001_a407" => Some(tig_algorithms::satisfiability::c001_a407::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a408")]
                                        // "c001_a408" => Some(tig_algorithms::satisfiability::c001_a408::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a409")]
                                        // "c001_a409" => Some(tig_algorithms::satisfiability::c001_a409::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a410")]
                                        // "c001_a410" => Some(tig_algorithms::satisfiability::c001_a410::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a411")]
                                        // "c001_a411" => Some(tig_algorithms::satisfiability::c001_a411::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a412")]
                                        // "c001_a412" => Some(tig_algorithms::satisfiability::c001_a412::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a413")]
                                        // "c001_a413" => Some(tig_algorithms::satisfiability::c001_a413::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a414")]
                                        // "c001_a414" => Some(tig_algorithms::satisfiability::c001_a414::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a415")]
                                        // "c001_a415" => Some(tig_algorithms::satisfiability::c001_a415::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a416")]
                                        // "c001_a416" => Some(tig_algorithms::satisfiability::c001_a416::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a417")]
                                        // "c001_a417" => Some(tig_algorithms::satisfiability::c001_a417::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a418")]
                                        // "c001_a418" => Some(tig_algorithms::satisfiability::c001_a418::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a419")]
                                        // "c001_a419" => Some(tig_algorithms::satisfiability::c001_a419::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a420")]
                                        // "c001_a420" => Some(tig_algorithms::satisfiability::c001_a420::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a421")]
                                        // "c001_a421" => Some(tig_algorithms::satisfiability::c001_a421::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a422")]
                                        // "c001_a422" => Some(tig_algorithms::satisfiability::c001_a422::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a423")]
                                        // "c001_a423" => Some(tig_algorithms::satisfiability::c001_a423::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a424")]
                                        // "c001_a424" => Some(tig_algorithms::satisfiability::c001_a424::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a425")]
                                        // "c001_a425" => Some(tig_algorithms::satisfiability::c001_a425::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a426")]
                                        // "c001_a426" => Some(tig_algorithms::satisfiability::c001_a426::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a427")]
                                        // "c001_a427" => Some(tig_algorithms::satisfiability::c001_a427::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a428")]
                                        // "c001_a428" => Some(tig_algorithms::satisfiability::c001_a428::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a429")]
                                        // "c001_a429" => Some(tig_algorithms::satisfiability::c001_a429::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a430")]
                                        // "c001_a430" => Some(tig_algorithms::satisfiability::c001_a430::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a431")]
                                        // "c001_a431" => Some(tig_algorithms::satisfiability::c001_a431::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a432")]
                                        // "c001_a432" => Some(tig_algorithms::satisfiability::c001_a432::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a433")]
                                        // "c001_a433" => Some(tig_algorithms::satisfiability::c001_a433::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a434")]
                                        // "c001_a434" => Some(tig_algorithms::satisfiability::c001_a434::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a435")]
                                        // "c001_a435" => Some(tig_algorithms::satisfiability::c001_a435::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a436")]
                                        // "c001_a436" => Some(tig_algorithms::satisfiability::c001_a436::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a437")]
                                        // "c001_a437" => Some(tig_algorithms::satisfiability::c001_a437::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a438")]
                                        // "c001_a438" => Some(tig_algorithms::satisfiability::c001_a438::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a439")]
                                        // "c001_a439" => Some(tig_algorithms::satisfiability::c001_a439::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a440")]
                                        // "c001_a440" => Some(tig_algorithms::satisfiability::c001_a440::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a441")]
                                        // "c001_a441" => Some(tig_algorithms::satisfiability::c001_a441::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a442")]
                                        // "c001_a442" => Some(tig_algorithms::satisfiability::c001_a442::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a443")]
                                        // "c001_a443" => Some(tig_algorithms::satisfiability::c001_a443::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a444")]
                                        // "c001_a444" => Some(tig_algorithms::satisfiability::c001_a444::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a445")]
                                        // "c001_a445" => Some(tig_algorithms::satisfiability::c001_a445::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a446")]
                                        // "c001_a446" => Some(tig_algorithms::satisfiability::c001_a446::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a447")]
                                        // "c001_a447" => Some(tig_algorithms::satisfiability::c001_a447::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a448")]
                                        // "c001_a448" => Some(tig_algorithms::satisfiability::c001_a448::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a449")]
                                        // "c001_a449" => Some(tig_algorithms::satisfiability::c001_a449::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a450")]
                                        // "c001_a450" => Some(tig_algorithms::satisfiability::c001_a450::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a451")]
                                        // "c001_a451" => Some(tig_algorithms::satisfiability::c001_a451::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a452")]
                                        // "c001_a452" => Some(tig_algorithms::satisfiability::c001_a452::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a453")]
                                        // "c001_a453" => Some(tig_algorithms::satisfiability::c001_a453::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a454")]
                                        // "c001_a454" => Some(tig_algorithms::satisfiability::c001_a454::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a455")]
                                        // "c001_a455" => Some(tig_algorithms::satisfiability::c001_a455::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a456")]
                                        // "c001_a456" => Some(tig_algorithms::satisfiability::c001_a456::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a457")]
                                        // "c001_a457" => Some(tig_algorithms::satisfiability::c001_a457::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a458")]
                                        // "c001_a458" => Some(tig_algorithms::satisfiability::c001_a458::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a459")]
                                        // "c001_a459" => Some(tig_algorithms::satisfiability::c001_a459::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a460")]
                                        // "c001_a460" => Some(tig_algorithms::satisfiability::c001_a460::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a461")]
                                        // "c001_a461" => Some(tig_algorithms::satisfiability::c001_a461::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a462")]
                                        // "c001_a462" => Some(tig_algorithms::satisfiability::c001_a462::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a463")]
                                        // "c001_a463" => Some(tig_algorithms::satisfiability::c001_a463::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a464")]
                                        // "c001_a464" => Some(tig_algorithms::satisfiability::c001_a464::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a465")]
                                        // "c001_a465" => Some(tig_algorithms::satisfiability::c001_a465::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a466")]
                                        // "c001_a466" => Some(tig_algorithms::satisfiability::c001_a466::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a467")]
                                        // "c001_a467" => Some(tig_algorithms::satisfiability::c001_a467::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a468")]
                                        // "c001_a468" => Some(tig_algorithms::satisfiability::c001_a468::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a469")]
                                        // "c001_a469" => Some(tig_algorithms::satisfiability::c001_a469::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a470")]
                                        // "c001_a470" => Some(tig_algorithms::satisfiability::c001_a470::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a471")]
                                        // "c001_a471" => Some(tig_algorithms::satisfiability::c001_a471::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a472")]
                                        // "c001_a472" => Some(tig_algorithms::satisfiability::c001_a472::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a473")]
                                        // "c001_a473" => Some(tig_algorithms::satisfiability::c001_a473::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a474")]
                                        // "c001_a474" => Some(tig_algorithms::satisfiability::c001_a474::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a475")]
                                        // "c001_a475" => Some(tig_algorithms::satisfiability::c001_a475::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a476")]
                                        // "c001_a476" => Some(tig_algorithms::satisfiability::c001_a476::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a477")]
                                        // "c001_a477" => Some(tig_algorithms::satisfiability::c001_a477::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a478")]
                                        // "c001_a478" => Some(tig_algorithms::satisfiability::c001_a478::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a479")]
                                        // "c001_a479" => Some(tig_algorithms::satisfiability::c001_a479::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a480")]
                                        // "c001_a480" => Some(tig_algorithms::satisfiability::c001_a480::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a481")]
                                        // "c001_a481" => Some(tig_algorithms::satisfiability::c001_a481::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a482")]
                                        // "c001_a482" => Some(tig_algorithms::satisfiability::c001_a482::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a483")]
                                        // "c001_a483" => Some(tig_algorithms::satisfiability::c001_a483::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a484")]
                                        // "c001_a484" => Some(tig_algorithms::satisfiability::c001_a484::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a485")]
                                        // "c001_a485" => Some(tig_algorithms::satisfiability::c001_a485::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a486")]
                                        // "c001_a486" => Some(tig_algorithms::satisfiability::c001_a486::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a487")]
                                        // "c001_a487" => Some(tig_algorithms::satisfiability::c001_a487::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a488")]
                                        // "c001_a488" => Some(tig_algorithms::satisfiability::c001_a488::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a489")]
                                        // "c001_a489" => Some(tig_algorithms::satisfiability::c001_a489::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a490")]
                                        // "c001_a490" => Some(tig_algorithms::satisfiability::c001_a490::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a491")]
                                        // "c001_a491" => Some(tig_algorithms::satisfiability::c001_a491::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a492")]
                                        // "c001_a492" => Some(tig_algorithms::satisfiability::c001_a492::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a493")]
                                        // "c001_a493" => Some(tig_algorithms::satisfiability::c001_a493::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a494")]
                                        // "c001_a494" => Some(tig_algorithms::satisfiability::c001_a494::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a495")]
                                        // "c001_a495" => Some(tig_algorithms::satisfiability::c001_a495::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a496")]
                                        // "c001_a496" => Some(tig_algorithms::satisfiability::c001_a496::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a497")]
                                        // "c001_a497" => Some(tig_algorithms::satisfiability::c001_a497::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a498")]
                                        // "c001_a498" => Some(tig_algorithms::satisfiability::c001_a498::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a499")]
                                        // "c001_a499" => Some(tig_algorithms::satisfiability::c001_a499::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a500")]
                                        // "c001_a500" => Some(tig_algorithms::satisfiability::c001_a500::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a501")]
                                        // "c001_a501" => Some(tig_algorithms::satisfiability::c001_a501::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a502")]
                                        // "c001_a502" => Some(tig_algorithms::satisfiability::c001_a502::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a503")]
                                        // "c001_a503" => Some(tig_algorithms::satisfiability::c001_a503::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a504")]
                                        // "c001_a504" => Some(tig_algorithms::satisfiability::c001_a504::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a505")]
                                        // "c001_a505" => Some(tig_algorithms::satisfiability::c001_a505::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a506")]
                                        // "c001_a506" => Some(tig_algorithms::satisfiability::c001_a506::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a507")]
                                        // "c001_a507" => Some(tig_algorithms::satisfiability::c001_a507::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a508")]
                                        // "c001_a508" => Some(tig_algorithms::satisfiability::c001_a508::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a509")]
                                        // "c001_a509" => Some(tig_algorithms::satisfiability::c001_a509::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a510")]
                                        // "c001_a510" => Some(tig_algorithms::satisfiability::c001_a510::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a511")]
                                        // "c001_a511" => Some(tig_algorithms::satisfiability::c001_a511::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a512")]
                                        // "c001_a512" => Some(tig_algorithms::satisfiability::c001_a512::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a513")]
                                        // "c001_a513" => Some(tig_algorithms::satisfiability::c001_a513::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a514")]
                                        // "c001_a514" => Some(tig_algorithms::satisfiability::c001_a514::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a515")]
                                        // "c001_a515" => Some(tig_algorithms::satisfiability::c001_a515::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a516")]
                                        // "c001_a516" => Some(tig_algorithms::satisfiability::c001_a516::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a517")]
                                        // "c001_a517" => Some(tig_algorithms::satisfiability::c001_a517::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a518")]
                                        // "c001_a518" => Some(tig_algorithms::satisfiability::c001_a518::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a519")]
                                        // "c001_a519" => Some(tig_algorithms::satisfiability::c001_a519::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a520")]
                                        // "c001_a520" => Some(tig_algorithms::satisfiability::c001_a520::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a521")]
                                        // "c001_a521" => Some(tig_algorithms::satisfiability::c001_a521::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a522")]
                                        // "c001_a522" => Some(tig_algorithms::satisfiability::c001_a522::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a523")]
                                        // "c001_a523" => Some(tig_algorithms::satisfiability::c001_a523::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a524")]
                                        // "c001_a524" => Some(tig_algorithms::satisfiability::c001_a524::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a525")]
                                        // "c001_a525" => Some(tig_algorithms::satisfiability::c001_a525::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a526")]
                                        // "c001_a526" => Some(tig_algorithms::satisfiability::c001_a526::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a527")]
                                        // "c001_a527" => Some(tig_algorithms::satisfiability::c001_a527::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a528")]
                                        // "c001_a528" => Some(tig_algorithms::satisfiability::c001_a528::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a529")]
                                        // "c001_a529" => Some(tig_algorithms::satisfiability::c001_a529::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a530")]
                                        // "c001_a530" => Some(tig_algorithms::satisfiability::c001_a530::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a531")]
                                        // "c001_a531" => Some(tig_algorithms::satisfiability::c001_a531::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a532")]
                                        // "c001_a532" => Some(tig_algorithms::satisfiability::c001_a532::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a533")]
                                        // "c001_a533" => Some(tig_algorithms::satisfiability::c001_a533::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a534")]
                                        // "c001_a534" => Some(tig_algorithms::satisfiability::c001_a534::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a535")]
                                        // "c001_a535" => Some(tig_algorithms::satisfiability::c001_a535::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a536")]
                                        // "c001_a536" => Some(tig_algorithms::satisfiability::c001_a536::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a537")]
                                        // "c001_a537" => Some(tig_algorithms::satisfiability::c001_a537::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a538")]
                                        // "c001_a538" => Some(tig_algorithms::satisfiability::c001_a538::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a539")]
                                        // "c001_a539" => Some(tig_algorithms::satisfiability::c001_a539::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a540")]
                                        // "c001_a540" => Some(tig_algorithms::satisfiability::c001_a540::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a541")]
                                        // "c001_a541" => Some(tig_algorithms::satisfiability::c001_a541::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a542")]
                                        // "c001_a542" => Some(tig_algorithms::satisfiability::c001_a542::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a543")]
                                        // "c001_a543" => Some(tig_algorithms::satisfiability::c001_a543::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a544")]
                                        // "c001_a544" => Some(tig_algorithms::satisfiability::c001_a544::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a545")]
                                        // "c001_a545" => Some(tig_algorithms::satisfiability::c001_a545::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a546")]
                                        // "c001_a546" => Some(tig_algorithms::satisfiability::c001_a546::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a547")]
                                        // "c001_a547" => Some(tig_algorithms::satisfiability::c001_a547::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a548")]
                                        // "c001_a548" => Some(tig_algorithms::satisfiability::c001_a548::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a549")]
                                        // "c001_a549" => Some(tig_algorithms::satisfiability::c001_a549::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a550")]
                                        // "c001_a550" => Some(tig_algorithms::satisfiability::c001_a550::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a551")]
                                        // "c001_a551" => Some(tig_algorithms::satisfiability::c001_a551::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a552")]
                                        // "c001_a552" => Some(tig_algorithms::satisfiability::c001_a552::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a553")]
                                        // "c001_a553" => Some(tig_algorithms::satisfiability::c001_a553::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a554")]
                                        // "c001_a554" => Some(tig_algorithms::satisfiability::c001_a554::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a555")]
                                        // "c001_a555" => Some(tig_algorithms::satisfiability::c001_a555::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a556")]
                                        // "c001_a556" => Some(tig_algorithms::satisfiability::c001_a556::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a557")]
                                        // "c001_a557" => Some(tig_algorithms::satisfiability::c001_a557::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a558")]
                                        // "c001_a558" => Some(tig_algorithms::satisfiability::c001_a558::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a559")]
                                        // "c001_a559" => Some(tig_algorithms::satisfiability::c001_a559::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a560")]
                                        // "c001_a560" => Some(tig_algorithms::satisfiability::c001_a560::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a561")]
                                        // "c001_a561" => Some(tig_algorithms::satisfiability::c001_a561::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a562")]
                                        // "c001_a562" => Some(tig_algorithms::satisfiability::c001_a562::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a563")]
                                        // "c001_a563" => Some(tig_algorithms::satisfiability::c001_a563::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a564")]
                                        // "c001_a564" => Some(tig_algorithms::satisfiability::c001_a564::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a565")]
                                        // "c001_a565" => Some(tig_algorithms::satisfiability::c001_a565::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a566")]
                                        // "c001_a566" => Some(tig_algorithms::satisfiability::c001_a566::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a567")]
                                        // "c001_a567" => Some(tig_algorithms::satisfiability::c001_a567::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a568")]
                                        // "c001_a568" => Some(tig_algorithms::satisfiability::c001_a568::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a569")]
                                        // "c001_a569" => Some(tig_algorithms::satisfiability::c001_a569::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a570")]
                                        // "c001_a570" => Some(tig_algorithms::satisfiability::c001_a570::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a571")]
                                        // "c001_a571" => Some(tig_algorithms::satisfiability::c001_a571::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a572")]
                                        // "c001_a572" => Some(tig_algorithms::satisfiability::c001_a572::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a573")]
                                        // "c001_a573" => Some(tig_algorithms::satisfiability::c001_a573::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a574")]
                                        // "c001_a574" => Some(tig_algorithms::satisfiability::c001_a574::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a575")]
                                        // "c001_a575" => Some(tig_algorithms::satisfiability::c001_a575::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a576")]
                                        // "c001_a576" => Some(tig_algorithms::satisfiability::c001_a576::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a577")]
                                        // "c001_a577" => Some(tig_algorithms::satisfiability::c001_a577::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a578")]
                                        // "c001_a578" => Some(tig_algorithms::satisfiability::c001_a578::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a579")]
                                        // "c001_a579" => Some(tig_algorithms::satisfiability::c001_a579::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a580")]
                                        // "c001_a580" => Some(tig_algorithms::satisfiability::c001_a580::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a581")]
                                        // "c001_a581" => Some(tig_algorithms::satisfiability::c001_a581::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a582")]
                                        // "c001_a582" => Some(tig_algorithms::satisfiability::c001_a582::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a583")]
                                        // "c001_a583" => Some(tig_algorithms::satisfiability::c001_a583::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a584")]
                                        // "c001_a584" => Some(tig_algorithms::satisfiability::c001_a584::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a585")]
                                        // "c001_a585" => Some(tig_algorithms::satisfiability::c001_a585::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a586")]
                                        // "c001_a586" => Some(tig_algorithms::satisfiability::c001_a586::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a587")]
                                        // "c001_a587" => Some(tig_algorithms::satisfiability::c001_a587::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a588")]
                                        // "c001_a588" => Some(tig_algorithms::satisfiability::c001_a588::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a589")]
                                        // "c001_a589" => Some(tig_algorithms::satisfiability::c001_a589::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a590")]
                                        // "c001_a590" => Some(tig_algorithms::satisfiability::c001_a590::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a591")]
                                        // "c001_a591" => Some(tig_algorithms::satisfiability::c001_a591::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a592")]
                                        // "c001_a592" => Some(tig_algorithms::satisfiability::c001_a592::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a593")]
                                        // "c001_a593" => Some(tig_algorithms::satisfiability::c001_a593::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a594")]
                                        // "c001_a594" => Some(tig_algorithms::satisfiability::c001_a594::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a595")]
                                        // "c001_a595" => Some(tig_algorithms::satisfiability::c001_a595::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a596")]
                                        // "c001_a596" => Some(tig_algorithms::satisfiability::c001_a596::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a597")]
                                        // "c001_a597" => Some(tig_algorithms::satisfiability::c001_a597::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a598")]
                                        // "c001_a598" => Some(tig_algorithms::satisfiability::c001_a598::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a599")]
                                        // "c001_a599" => Some(tig_algorithms::satisfiability::c001_a599::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a600")]
                                        // "c001_a600" => Some(tig_algorithms::satisfiability::c001_a600::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a601")]
                                        // "c001_a601" => Some(tig_algorithms::satisfiability::c001_a601::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a602")]
                                        // "c001_a602" => Some(tig_algorithms::satisfiability::c001_a602::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a603")]
                                        // "c001_a603" => Some(tig_algorithms::satisfiability::c001_a603::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a604")]
                                        // "c001_a604" => Some(tig_algorithms::satisfiability::c001_a604::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a605")]
                                        // "c001_a605" => Some(tig_algorithms::satisfiability::c001_a605::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a606")]
                                        // "c001_a606" => Some(tig_algorithms::satisfiability::c001_a606::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a607")]
                                        // "c001_a607" => Some(tig_algorithms::satisfiability::c001_a607::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a608")]
                                        // "c001_a608" => Some(tig_algorithms::satisfiability::c001_a608::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a609")]
                                        // "c001_a609" => Some(tig_algorithms::satisfiability::c001_a609::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a610")]
                                        // "c001_a610" => Some(tig_algorithms::satisfiability::c001_a610::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a611")]
                                        // "c001_a611" => Some(tig_algorithms::satisfiability::c001_a611::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a612")]
                                        // "c001_a612" => Some(tig_algorithms::satisfiability::c001_a612::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a613")]
                                        // "c001_a613" => Some(tig_algorithms::satisfiability::c001_a613::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a614")]
                                        // "c001_a614" => Some(tig_algorithms::satisfiability::c001_a614::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a615")]
                                        // "c001_a615" => Some(tig_algorithms::satisfiability::c001_a615::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a616")]
                                        // "c001_a616" => Some(tig_algorithms::satisfiability::c001_a616::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a617")]
                                        // "c001_a617" => Some(tig_algorithms::satisfiability::c001_a617::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a618")]
                                        // "c001_a618" => Some(tig_algorithms::satisfiability::c001_a618::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a619")]
                                        // "c001_a619" => Some(tig_algorithms::satisfiability::c001_a619::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a620")]
                                        // "c001_a620" => Some(tig_algorithms::satisfiability::c001_a620::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a621")]
                                        // "c001_a621" => Some(tig_algorithms::satisfiability::c001_a621::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a622")]
                                        // "c001_a622" => Some(tig_algorithms::satisfiability::c001_a622::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a623")]
                                        // "c001_a623" => Some(tig_algorithms::satisfiability::c001_a623::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a624")]
                                        // "c001_a624" => Some(tig_algorithms::satisfiability::c001_a624::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a625")]
                                        // "c001_a625" => Some(tig_algorithms::satisfiability::c001_a625::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a626")]
                                        // "c001_a626" => Some(tig_algorithms::satisfiability::c001_a626::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a627")]
                                        // "c001_a627" => Some(tig_algorithms::satisfiability::c001_a627::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a628")]
                                        // "c001_a628" => Some(tig_algorithms::satisfiability::c001_a628::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a629")]
                                        // "c001_a629" => Some(tig_algorithms::satisfiability::c001_a629::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a630")]
                                        // "c001_a630" => Some(tig_algorithms::satisfiability::c001_a630::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a631")]
                                        // "c001_a631" => Some(tig_algorithms::satisfiability::c001_a631::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a632")]
                                        // "c001_a632" => Some(tig_algorithms::satisfiability::c001_a632::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a633")]
                                        // "c001_a633" => Some(tig_algorithms::satisfiability::c001_a633::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a634")]
                                        // "c001_a634" => Some(tig_algorithms::satisfiability::c001_a634::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a635")]
                                        // "c001_a635" => Some(tig_algorithms::satisfiability::c001_a635::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a636")]
                                        // "c001_a636" => Some(tig_algorithms::satisfiability::c001_a636::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a637")]
                                        // "c001_a637" => Some(tig_algorithms::satisfiability::c001_a637::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a638")]
                                        // "c001_a638" => Some(tig_algorithms::satisfiability::c001_a638::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a639")]
                                        // "c001_a639" => Some(tig_algorithms::satisfiability::c001_a639::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a640")]
                                        // "c001_a640" => Some(tig_algorithms::satisfiability::c001_a640::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a641")]
                                        // "c001_a641" => Some(tig_algorithms::satisfiability::c001_a641::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a642")]
                                        // "c001_a642" => Some(tig_algorithms::satisfiability::c001_a642::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a643")]
                                        // "c001_a643" => Some(tig_algorithms::satisfiability::c001_a643::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a644")]
                                        // "c001_a644" => Some(tig_algorithms::satisfiability::c001_a644::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a645")]
                                        // "c001_a645" => Some(tig_algorithms::satisfiability::c001_a645::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a646")]
                                        // "c001_a646" => Some(tig_algorithms::satisfiability::c001_a646::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a647")]
                                        // "c001_a647" => Some(tig_algorithms::satisfiability::c001_a647::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a648")]
                                        // "c001_a648" => Some(tig_algorithms::satisfiability::c001_a648::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a649")]
                                        // "c001_a649" => Some(tig_algorithms::satisfiability::c001_a649::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a650")]
                                        // "c001_a650" => Some(tig_algorithms::satisfiability::c001_a650::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a651")]
                                        // "c001_a651" => Some(tig_algorithms::satisfiability::c001_a651::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a652")]
                                        // "c001_a652" => Some(tig_algorithms::satisfiability::c001_a652::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a653")]
                                        // "c001_a653" => Some(tig_algorithms::satisfiability::c001_a653::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a654")]
                                        // "c001_a654" => Some(tig_algorithms::satisfiability::c001_a654::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a655")]
                                        // "c001_a655" => Some(tig_algorithms::satisfiability::c001_a655::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a656")]
                                        // "c001_a656" => Some(tig_algorithms::satisfiability::c001_a656::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a657")]
                                        // "c001_a657" => Some(tig_algorithms::satisfiability::c001_a657::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a658")]
                                        // "c001_a658" => Some(tig_algorithms::satisfiability::c001_a658::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a659")]
                                        // "c001_a659" => Some(tig_algorithms::satisfiability::c001_a659::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a660")]
                                        // "c001_a660" => Some(tig_algorithms::satisfiability::c001_a660::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a661")]
                                        // "c001_a661" => Some(tig_algorithms::satisfiability::c001_a661::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a662")]
                                        // "c001_a662" => Some(tig_algorithms::satisfiability::c001_a662::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a663")]
                                        // "c001_a663" => Some(tig_algorithms::satisfiability::c001_a663::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a664")]
                                        // "c001_a664" => Some(tig_algorithms::satisfiability::c001_a664::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a665")]
                                        // "c001_a665" => Some(tig_algorithms::satisfiability::c001_a665::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a666")]
                                        // "c001_a666" => Some(tig_algorithms::satisfiability::c001_a666::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a667")]
                                        // "c001_a667" => Some(tig_algorithms::satisfiability::c001_a667::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a668")]
                                        // "c001_a668" => Some(tig_algorithms::satisfiability::c001_a668::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a669")]
                                        // "c001_a669" => Some(tig_algorithms::satisfiability::c001_a669::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a670")]
                                        // "c001_a670" => Some(tig_algorithms::satisfiability::c001_a670::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a671")]
                                        // "c001_a671" => Some(tig_algorithms::satisfiability::c001_a671::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a672")]
                                        // "c001_a672" => Some(tig_algorithms::satisfiability::c001_a672::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a673")]
                                        // "c001_a673" => Some(tig_algorithms::satisfiability::c001_a673::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a674")]
                                        // "c001_a674" => Some(tig_algorithms::satisfiability::c001_a674::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a675")]
                                        // "c001_a675" => Some(tig_algorithms::satisfiability::c001_a675::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a676")]
                                        // "c001_a676" => Some(tig_algorithms::satisfiability::c001_a676::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a677")]
                                        // "c001_a677" => Some(tig_algorithms::satisfiability::c001_a677::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a678")]
                                        // "c001_a678" => Some(tig_algorithms::satisfiability::c001_a678::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a679")]
                                        // "c001_a679" => Some(tig_algorithms::satisfiability::c001_a679::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a680")]
                                        // "c001_a680" => Some(tig_algorithms::satisfiability::c001_a680::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a681")]
                                        // "c001_a681" => Some(tig_algorithms::satisfiability::c001_a681::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a682")]
                                        // "c001_a682" => Some(tig_algorithms::satisfiability::c001_a682::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a683")]
                                        // "c001_a683" => Some(tig_algorithms::satisfiability::c001_a683::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a684")]
                                        // "c001_a684" => Some(tig_algorithms::satisfiability::c001_a684::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a685")]
                                        // "c001_a685" => Some(tig_algorithms::satisfiability::c001_a685::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a686")]
                                        // "c001_a686" => Some(tig_algorithms::satisfiability::c001_a686::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a687")]
                                        // "c001_a687" => Some(tig_algorithms::satisfiability::c001_a687::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a688")]
                                        // "c001_a688" => Some(tig_algorithms::satisfiability::c001_a688::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a689")]
                                        // "c001_a689" => Some(tig_algorithms::satisfiability::c001_a689::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a690")]
                                        // "c001_a690" => Some(tig_algorithms::satisfiability::c001_a690::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a691")]
                                        // "c001_a691" => Some(tig_algorithms::satisfiability::c001_a691::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a692")]
                                        // "c001_a692" => Some(tig_algorithms::satisfiability::c001_a692::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a693")]
                                        // "c001_a693" => Some(tig_algorithms::satisfiability::c001_a693::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a694")]
                                        // "c001_a694" => Some(tig_algorithms::satisfiability::c001_a694::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a695")]
                                        // "c001_a695" => Some(tig_algorithms::satisfiability::c001_a695::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a696")]
                                        // "c001_a696" => Some(tig_algorithms::satisfiability::c001_a696::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a697")]
                                        // "c001_a697" => Some(tig_algorithms::satisfiability::c001_a697::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a698")]
                                        // "c001_a698" => Some(tig_algorithms::satisfiability::c001_a698::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a699")]
                                        // "c001_a699" => Some(tig_algorithms::satisfiability::c001_a699::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a700")]
                                        // "c001_a700" => Some(tig_algorithms::satisfiability::c001_a700::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a701")]
                                        // "c001_a701" => Some(tig_algorithms::satisfiability::c001_a701::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a702")]
                                        // "c001_a702" => Some(tig_algorithms::satisfiability::c001_a702::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a703")]
                                        // "c001_a703" => Some(tig_algorithms::satisfiability::c001_a703::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a704")]
                                        // "c001_a704" => Some(tig_algorithms::satisfiability::c001_a704::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a705")]
                                        // "c001_a705" => Some(tig_algorithms::satisfiability::c001_a705::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a706")]
                                        // "c001_a706" => Some(tig_algorithms::satisfiability::c001_a706::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a707")]
                                        // "c001_a707" => Some(tig_algorithms::satisfiability::c001_a707::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a708")]
                                        // "c001_a708" => Some(tig_algorithms::satisfiability::c001_a708::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a709")]
                                        // "c001_a709" => Some(tig_algorithms::satisfiability::c001_a709::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a710")]
                                        // "c001_a710" => Some(tig_algorithms::satisfiability::c001_a710::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a711")]
                                        // "c001_a711" => Some(tig_algorithms::satisfiability::c001_a711::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a712")]
                                        // "c001_a712" => Some(tig_algorithms::satisfiability::c001_a712::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a713")]
                                        // "c001_a713" => Some(tig_algorithms::satisfiability::c001_a713::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a714")]
                                        // "c001_a714" => Some(tig_algorithms::satisfiability::c001_a714::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a715")]
                                        // "c001_a715" => Some(tig_algorithms::satisfiability::c001_a715::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a716")]
                                        // "c001_a716" => Some(tig_algorithms::satisfiability::c001_a716::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a717")]
                                        // "c001_a717" => Some(tig_algorithms::satisfiability::c001_a717::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a718")]
                                        // "c001_a718" => Some(tig_algorithms::satisfiability::c001_a718::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a719")]
                                        // "c001_a719" => Some(tig_algorithms::satisfiability::c001_a719::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a720")]
                                        // "c001_a720" => Some(tig_algorithms::satisfiability::c001_a720::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a721")]
                                        // "c001_a721" => Some(tig_algorithms::satisfiability::c001_a721::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a722")]
                                        // "c001_a722" => Some(tig_algorithms::satisfiability::c001_a722::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a723")]
                                        // "c001_a723" => Some(tig_algorithms::satisfiability::c001_a723::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a724")]
                                        // "c001_a724" => Some(tig_algorithms::satisfiability::c001_a724::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a725")]
                                        // "c001_a725" => Some(tig_algorithms::satisfiability::c001_a725::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a726")]
                                        // "c001_a726" => Some(tig_algorithms::satisfiability::c001_a726::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a727")]
                                        // "c001_a727" => Some(tig_algorithms::satisfiability::c001_a727::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a728")]
                                        // "c001_a728" => Some(tig_algorithms::satisfiability::c001_a728::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a729")]
                                        // "c001_a729" => Some(tig_algorithms::satisfiability::c001_a729::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a730")]
                                        // "c001_a730" => Some(tig_algorithms::satisfiability::c001_a730::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a731")]
                                        // "c001_a731" => Some(tig_algorithms::satisfiability::c001_a731::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a732")]
                                        // "c001_a732" => Some(tig_algorithms::satisfiability::c001_a732::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a733")]
                                        // "c001_a733" => Some(tig_algorithms::satisfiability::c001_a733::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a734")]
                                        // "c001_a734" => Some(tig_algorithms::satisfiability::c001_a734::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a735")]
                                        // "c001_a735" => Some(tig_algorithms::satisfiability::c001_a735::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a736")]
                                        // "c001_a736" => Some(tig_algorithms::satisfiability::c001_a736::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a737")]
                                        // "c001_a737" => Some(tig_algorithms::satisfiability::c001_a737::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a738")]
                                        // "c001_a738" => Some(tig_algorithms::satisfiability::c001_a738::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a739")]
                                        // "c001_a739" => Some(tig_algorithms::satisfiability::c001_a739::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a740")]
                                        // "c001_a740" => Some(tig_algorithms::satisfiability::c001_a740::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a741")]
                                        // "c001_a741" => Some(tig_algorithms::satisfiability::c001_a741::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a742")]
                                        // "c001_a742" => Some(tig_algorithms::satisfiability::c001_a742::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a743")]
                                        // "c001_a743" => Some(tig_algorithms::satisfiability::c001_a743::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a744")]
                                        // "c001_a744" => Some(tig_algorithms::satisfiability::c001_a744::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a745")]
                                        // "c001_a745" => Some(tig_algorithms::satisfiability::c001_a745::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a746")]
                                        // "c001_a746" => Some(tig_algorithms::satisfiability::c001_a746::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a747")]
                                        // "c001_a747" => Some(tig_algorithms::satisfiability::c001_a747::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a748")]
                                        // "c001_a748" => Some(tig_algorithms::satisfiability::c001_a748::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a749")]
                                        // "c001_a749" => Some(tig_algorithms::satisfiability::c001_a749::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a750")]
                                        // "c001_a750" => Some(tig_algorithms::satisfiability::c001_a750::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a751")]
                                        // "c001_a751" => Some(tig_algorithms::satisfiability::c001_a751::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a752")]
                                        // "c001_a752" => Some(tig_algorithms::satisfiability::c001_a752::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a753")]
                                        // "c001_a753" => Some(tig_algorithms::satisfiability::c001_a753::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a754")]
                                        // "c001_a754" => Some(tig_algorithms::satisfiability::c001_a754::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a755")]
                                        // "c001_a755" => Some(tig_algorithms::satisfiability::c001_a755::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a756")]
                                        // "c001_a756" => Some(tig_algorithms::satisfiability::c001_a756::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a757")]
                                        // "c001_a757" => Some(tig_algorithms::satisfiability::c001_a757::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a758")]
                                        // "c001_a758" => Some(tig_algorithms::satisfiability::c001_a758::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a759")]
                                        // "c001_a759" => Some(tig_algorithms::satisfiability::c001_a759::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a760")]
                                        // "c001_a760" => Some(tig_algorithms::satisfiability::c001_a760::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a761")]
                                        // "c001_a761" => Some(tig_algorithms::satisfiability::c001_a761::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a762")]
                                        // "c001_a762" => Some(tig_algorithms::satisfiability::c001_a762::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a763")]
                                        // "c001_a763" => Some(tig_algorithms::satisfiability::c001_a763::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a764")]
                                        // "c001_a764" => Some(tig_algorithms::satisfiability::c001_a764::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a765")]
                                        // "c001_a765" => Some(tig_algorithms::satisfiability::c001_a765::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a766")]
                                        // "c001_a766" => Some(tig_algorithms::satisfiability::c001_a766::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a767")]
                                        // "c001_a767" => Some(tig_algorithms::satisfiability::c001_a767::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a768")]
                                        // "c001_a768" => Some(tig_algorithms::satisfiability::c001_a768::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a769")]
                                        // "c001_a769" => Some(tig_algorithms::satisfiability::c001_a769::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a770")]
                                        // "c001_a770" => Some(tig_algorithms::satisfiability::c001_a770::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a771")]
                                        // "c001_a771" => Some(tig_algorithms::satisfiability::c001_a771::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a772")]
                                        // "c001_a772" => Some(tig_algorithms::satisfiability::c001_a772::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a773")]
                                        // "c001_a773" => Some(tig_algorithms::satisfiability::c001_a773::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a774")]
                                        // "c001_a774" => Some(tig_algorithms::satisfiability::c001_a774::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a775")]
                                        // "c001_a775" => Some(tig_algorithms::satisfiability::c001_a775::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a776")]
                                        // "c001_a776" => Some(tig_algorithms::satisfiability::c001_a776::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a777")]
                                        // "c001_a777" => Some(tig_algorithms::satisfiability::c001_a777::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a778")]
                                        // "c001_a778" => Some(tig_algorithms::satisfiability::c001_a778::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a779")]
                                        // "c001_a779" => Some(tig_algorithms::satisfiability::c001_a779::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a780")]
                                        // "c001_a780" => Some(tig_algorithms::satisfiability::c001_a780::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a781")]
                                        // "c001_a781" => Some(tig_algorithms::satisfiability::c001_a781::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a782")]
                                        // "c001_a782" => Some(tig_algorithms::satisfiability::c001_a782::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a783")]
                                        // "c001_a783" => Some(tig_algorithms::satisfiability::c001_a783::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a784")]
                                        // "c001_a784" => Some(tig_algorithms::satisfiability::c001_a784::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a785")]
                                        // "c001_a785" => Some(tig_algorithms::satisfiability::c001_a785::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a786")]
                                        // "c001_a786" => Some(tig_algorithms::satisfiability::c001_a786::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a787")]
                                        // "c001_a787" => Some(tig_algorithms::satisfiability::c001_a787::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a788")]
                                        // "c001_a788" => Some(tig_algorithms::satisfiability::c001_a788::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a789")]
                                        // "c001_a789" => Some(tig_algorithms::satisfiability::c001_a789::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a790")]
                                        // "c001_a790" => Some(tig_algorithms::satisfiability::c001_a790::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a791")]
                                        // "c001_a791" => Some(tig_algorithms::satisfiability::c001_a791::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a792")]
                                        // "c001_a792" => Some(tig_algorithms::satisfiability::c001_a792::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a793")]
                                        // "c001_a793" => Some(tig_algorithms::satisfiability::c001_a793::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a794")]
                                        // "c001_a794" => Some(tig_algorithms::satisfiability::c001_a794::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a795")]
                                        // "c001_a795" => Some(tig_algorithms::satisfiability::c001_a795::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a796")]
                                        // "c001_a796" => Some(tig_algorithms::satisfiability::c001_a796::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a797")]
                                        // "c001_a797" => Some(tig_algorithms::satisfiability::c001_a797::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a798")]
                                        // "c001_a798" => Some(tig_algorithms::satisfiability::c001_a798::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a799")]
                                        // "c001_a799" => Some(tig_algorithms::satisfiability::c001_a799::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a800")]
                                        // "c001_a800" => Some(tig_algorithms::satisfiability::c001_a800::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a801")]
                                        // "c001_a801" => Some(tig_algorithms::satisfiability::c001_a801::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a802")]
                                        // "c001_a802" => Some(tig_algorithms::satisfiability::c001_a802::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a803")]
                                        // "c001_a803" => Some(tig_algorithms::satisfiability::c001_a803::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a804")]
                                        // "c001_a804" => Some(tig_algorithms::satisfiability::c001_a804::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a805")]
                                        // "c001_a805" => Some(tig_algorithms::satisfiability::c001_a805::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a806")]
                                        // "c001_a806" => Some(tig_algorithms::satisfiability::c001_a806::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a807")]
                                        // "c001_a807" => Some(tig_algorithms::satisfiability::c001_a807::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a808")]
                                        // "c001_a808" => Some(tig_algorithms::satisfiability::c001_a808::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a809")]
                                        // "c001_a809" => Some(tig_algorithms::satisfiability::c001_a809::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a810")]
                                        // "c001_a810" => Some(tig_algorithms::satisfiability::c001_a810::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a811")]
                                        // "c001_a811" => Some(tig_algorithms::satisfiability::c001_a811::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a812")]
                                        // "c001_a812" => Some(tig_algorithms::satisfiability::c001_a812::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a813")]
                                        // "c001_a813" => Some(tig_algorithms::satisfiability::c001_a813::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a814")]
                                        // "c001_a814" => Some(tig_algorithms::satisfiability::c001_a814::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a815")]
                                        // "c001_a815" => Some(tig_algorithms::satisfiability::c001_a815::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a816")]
                                        // "c001_a816" => Some(tig_algorithms::satisfiability::c001_a816::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a817")]
                                        // "c001_a817" => Some(tig_algorithms::satisfiability::c001_a817::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a818")]
                                        // "c001_a818" => Some(tig_algorithms::satisfiability::c001_a818::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a819")]
                                        // "c001_a819" => Some(tig_algorithms::satisfiability::c001_a819::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a820")]
                                        // "c001_a820" => Some(tig_algorithms::satisfiability::c001_a820::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a821")]
                                        // "c001_a821" => Some(tig_algorithms::satisfiability::c001_a821::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a822")]
                                        // "c001_a822" => Some(tig_algorithms::satisfiability::c001_a822::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a823")]
                                        // "c001_a823" => Some(tig_algorithms::satisfiability::c001_a823::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a824")]
                                        // "c001_a824" => Some(tig_algorithms::satisfiability::c001_a824::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a825")]
                                        // "c001_a825" => Some(tig_algorithms::satisfiability::c001_a825::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a826")]
                                        // "c001_a826" => Some(tig_algorithms::satisfiability::c001_a826::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a827")]
                                        // "c001_a827" => Some(tig_algorithms::satisfiability::c001_a827::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a828")]
                                        // "c001_a828" => Some(tig_algorithms::satisfiability::c001_a828::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a829")]
                                        // "c001_a829" => Some(tig_algorithms::satisfiability::c001_a829::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a830")]
                                        // "c001_a830" => Some(tig_algorithms::satisfiability::c001_a830::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a831")]
                                        // "c001_a831" => Some(tig_algorithms::satisfiability::c001_a831::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a832")]
                                        // "c001_a832" => Some(tig_algorithms::satisfiability::c001_a832::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a833")]
                                        // "c001_a833" => Some(tig_algorithms::satisfiability::c001_a833::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a834")]
                                        // "c001_a834" => Some(tig_algorithms::satisfiability::c001_a834::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a835")]
                                        // "c001_a835" => Some(tig_algorithms::satisfiability::c001_a835::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a836")]
                                        // "c001_a836" => Some(tig_algorithms::satisfiability::c001_a836::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a837")]
                                        // "c001_a837" => Some(tig_algorithms::satisfiability::c001_a837::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a838")]
                                        // "c001_a838" => Some(tig_algorithms::satisfiability::c001_a838::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a839")]
                                        // "c001_a839" => Some(tig_algorithms::satisfiability::c001_a839::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a840")]
                                        // "c001_a840" => Some(tig_algorithms::satisfiability::c001_a840::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a841")]
                                        // "c001_a841" => Some(tig_algorithms::satisfiability::c001_a841::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a842")]
                                        // "c001_a842" => Some(tig_algorithms::satisfiability::c001_a842::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a843")]
                                        // "c001_a843" => Some(tig_algorithms::satisfiability::c001_a843::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a844")]
                                        // "c001_a844" => Some(tig_algorithms::satisfiability::c001_a844::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a845")]
                                        // "c001_a845" => Some(tig_algorithms::satisfiability::c001_a845::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a846")]
                                        // "c001_a846" => Some(tig_algorithms::satisfiability::c001_a846::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a847")]
                                        // "c001_a847" => Some(tig_algorithms::satisfiability::c001_a847::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a848")]
                                        // "c001_a848" => Some(tig_algorithms::satisfiability::c001_a848::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a849")]
                                        // "c001_a849" => Some(tig_algorithms::satisfiability::c001_a849::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a850")]
                                        // "c001_a850" => Some(tig_algorithms::satisfiability::c001_a850::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a851")]
                                        // "c001_a851" => Some(tig_algorithms::satisfiability::c001_a851::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a852")]
                                        // "c001_a852" => Some(tig_algorithms::satisfiability::c001_a852::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a853")]
                                        // "c001_a853" => Some(tig_algorithms::satisfiability::c001_a853::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a854")]
                                        // "c001_a854" => Some(tig_algorithms::satisfiability::c001_a854::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a855")]
                                        // "c001_a855" => Some(tig_algorithms::satisfiability::c001_a855::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a856")]
                                        // "c001_a856" => Some(tig_algorithms::satisfiability::c001_a856::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a857")]
                                        // "c001_a857" => Some(tig_algorithms::satisfiability::c001_a857::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a858")]
                                        // "c001_a858" => Some(tig_algorithms::satisfiability::c001_a858::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a859")]
                                        // "c001_a859" => Some(tig_algorithms::satisfiability::c001_a859::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a860")]
                                        // "c001_a860" => Some(tig_algorithms::satisfiability::c001_a860::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a861")]
                                        // "c001_a861" => Some(tig_algorithms::satisfiability::c001_a861::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a862")]
                                        // "c001_a862" => Some(tig_algorithms::satisfiability::c001_a862::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a863")]
                                        // "c001_a863" => Some(tig_algorithms::satisfiability::c001_a863::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a864")]
                                        // "c001_a864" => Some(tig_algorithms::satisfiability::c001_a864::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a865")]
                                        // "c001_a865" => Some(tig_algorithms::satisfiability::c001_a865::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a866")]
                                        // "c001_a866" => Some(tig_algorithms::satisfiability::c001_a866::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a867")]
                                        // "c001_a867" => Some(tig_algorithms::satisfiability::c001_a867::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a868")]
                                        // "c001_a868" => Some(tig_algorithms::satisfiability::c001_a868::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a869")]
                                        // "c001_a869" => Some(tig_algorithms::satisfiability::c001_a869::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a870")]
                                        // "c001_a870" => Some(tig_algorithms::satisfiability::c001_a870::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a871")]
                                        // "c001_a871" => Some(tig_algorithms::satisfiability::c001_a871::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a872")]
                                        // "c001_a872" => Some(tig_algorithms::satisfiability::c001_a872::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a873")]
                                        // "c001_a873" => Some(tig_algorithms::satisfiability::c001_a873::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a874")]
                                        // "c001_a874" => Some(tig_algorithms::satisfiability::c001_a874::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a875")]
                                        // "c001_a875" => Some(tig_algorithms::satisfiability::c001_a875::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a876")]
                                        // "c001_a876" => Some(tig_algorithms::satisfiability::c001_a876::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a877")]
                                        // "c001_a877" => Some(tig_algorithms::satisfiability::c001_a877::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a878")]
                                        // "c001_a878" => Some(tig_algorithms::satisfiability::c001_a878::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a879")]
                                        // "c001_a879" => Some(tig_algorithms::satisfiability::c001_a879::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a880")]
                                        // "c001_a880" => Some(tig_algorithms::satisfiability::c001_a880::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a881")]
                                        // "c001_a881" => Some(tig_algorithms::satisfiability::c001_a881::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a882")]
                                        // "c001_a882" => Some(tig_algorithms::satisfiability::c001_a882::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a883")]
                                        // "c001_a883" => Some(tig_algorithms::satisfiability::c001_a883::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a884")]
                                        // "c001_a884" => Some(tig_algorithms::satisfiability::c001_a884::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a885")]
                                        // "c001_a885" => Some(tig_algorithms::satisfiability::c001_a885::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a886")]
                                        // "c001_a886" => Some(tig_algorithms::satisfiability::c001_a886::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a887")]
                                        // "c001_a887" => Some(tig_algorithms::satisfiability::c001_a887::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a888")]
                                        // "c001_a888" => Some(tig_algorithms::satisfiability::c001_a888::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a889")]
                                        // "c001_a889" => Some(tig_algorithms::satisfiability::c001_a889::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a890")]
                                        // "c001_a890" => Some(tig_algorithms::satisfiability::c001_a890::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a891")]
                                        // "c001_a891" => Some(tig_algorithms::satisfiability::c001_a891::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a892")]
                                        // "c001_a892" => Some(tig_algorithms::satisfiability::c001_a892::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a893")]
                                        // "c001_a893" => Some(tig_algorithms::satisfiability::c001_a893::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a894")]
                                        // "c001_a894" => Some(tig_algorithms::satisfiability::c001_a894::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a895")]
                                        // "c001_a895" => Some(tig_algorithms::satisfiability::c001_a895::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a896")]
                                        // "c001_a896" => Some(tig_algorithms::satisfiability::c001_a896::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a897")]
                                        // "c001_a897" => Some(tig_algorithms::satisfiability::c001_a897::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a898")]
                                        // "c001_a898" => Some(tig_algorithms::satisfiability::c001_a898::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a899")]
                                        // "c001_a899" => Some(tig_algorithms::satisfiability::c001_a899::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a900")]
                                        // "c001_a900" => Some(tig_algorithms::satisfiability::c001_a900::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a901")]
                                        // "c001_a901" => Some(tig_algorithms::satisfiability::c001_a901::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a902")]
                                        // "c001_a902" => Some(tig_algorithms::satisfiability::c001_a902::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a903")]
                                        // "c001_a903" => Some(tig_algorithms::satisfiability::c001_a903::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a904")]
                                        // "c001_a904" => Some(tig_algorithms::satisfiability::c001_a904::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a905")]
                                        // "c001_a905" => Some(tig_algorithms::satisfiability::c001_a905::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a906")]
                                        // "c001_a906" => Some(tig_algorithms::satisfiability::c001_a906::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a907")]
                                        // "c001_a907" => Some(tig_algorithms::satisfiability::c001_a907::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a908")]
                                        // "c001_a908" => Some(tig_algorithms::satisfiability::c001_a908::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a909")]
                                        // "c001_a909" => Some(tig_algorithms::satisfiability::c001_a909::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a910")]
                                        // "c001_a910" => Some(tig_algorithms::satisfiability::c001_a910::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a911")]
                                        // "c001_a911" => Some(tig_algorithms::satisfiability::c001_a911::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a912")]
                                        // "c001_a912" => Some(tig_algorithms::satisfiability::c001_a912::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a913")]
                                        // "c001_a913" => Some(tig_algorithms::satisfiability::c001_a913::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a914")]
                                        // "c001_a914" => Some(tig_algorithms::satisfiability::c001_a914::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a915")]
                                        // "c001_a915" => Some(tig_algorithms::satisfiability::c001_a915::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a916")]
                                        // "c001_a916" => Some(tig_algorithms::satisfiability::c001_a916::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a917")]
                                        // "c001_a917" => Some(tig_algorithms::satisfiability::c001_a917::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a918")]
                                        // "c001_a918" => Some(tig_algorithms::satisfiability::c001_a918::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a919")]
                                        // "c001_a919" => Some(tig_algorithms::satisfiability::c001_a919::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a920")]
                                        // "c001_a920" => Some(tig_algorithms::satisfiability::c001_a920::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a921")]
                                        // "c001_a921" => Some(tig_algorithms::satisfiability::c001_a921::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a922")]
                                        // "c001_a922" => Some(tig_algorithms::satisfiability::c001_a922::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a923")]
                                        // "c001_a923" => Some(tig_algorithms::satisfiability::c001_a923::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a924")]
                                        // "c001_a924" => Some(tig_algorithms::satisfiability::c001_a924::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a925")]
                                        // "c001_a925" => Some(tig_algorithms::satisfiability::c001_a925::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a926")]
                                        // "c001_a926" => Some(tig_algorithms::satisfiability::c001_a926::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a927")]
                                        // "c001_a927" => Some(tig_algorithms::satisfiability::c001_a927::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a928")]
                                        // "c001_a928" => Some(tig_algorithms::satisfiability::c001_a928::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a929")]
                                        // "c001_a929" => Some(tig_algorithms::satisfiability::c001_a929::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a930")]
                                        // "c001_a930" => Some(tig_algorithms::satisfiability::c001_a930::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a931")]
                                        // "c001_a931" => Some(tig_algorithms::satisfiability::c001_a931::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a932")]
                                        // "c001_a932" => Some(tig_algorithms::satisfiability::c001_a932::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a933")]
                                        // "c001_a933" => Some(tig_algorithms::satisfiability::c001_a933::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a934")]
                                        // "c001_a934" => Some(tig_algorithms::satisfiability::c001_a934::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a935")]
                                        // "c001_a935" => Some(tig_algorithms::satisfiability::c001_a935::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a936")]
                                        // "c001_a936" => Some(tig_algorithms::satisfiability::c001_a936::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a937")]
                                        // "c001_a937" => Some(tig_algorithms::satisfiability::c001_a937::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a938")]
                                        // "c001_a938" => Some(tig_algorithms::satisfiability::c001_a938::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a939")]
                                        // "c001_a939" => Some(tig_algorithms::satisfiability::c001_a939::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a940")]
                                        // "c001_a940" => Some(tig_algorithms::satisfiability::c001_a940::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a941")]
                                        // "c001_a941" => Some(tig_algorithms::satisfiability::c001_a941::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a942")]
                                        // "c001_a942" => Some(tig_algorithms::satisfiability::c001_a942::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a943")]
                                        // "c001_a943" => Some(tig_algorithms::satisfiability::c001_a943::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a944")]
                                        // "c001_a944" => Some(tig_algorithms::satisfiability::c001_a944::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a945")]
                                        // "c001_a945" => Some(tig_algorithms::satisfiability::c001_a945::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a946")]
                                        // "c001_a946" => Some(tig_algorithms::satisfiability::c001_a946::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a947")]
                                        // "c001_a947" => Some(tig_algorithms::satisfiability::c001_a947::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a948")]
                                        // "c001_a948" => Some(tig_algorithms::satisfiability::c001_a948::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a949")]
                                        // "c001_a949" => Some(tig_algorithms::satisfiability::c001_a949::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a950")]
                                        // "c001_a950" => Some(tig_algorithms::satisfiability::c001_a950::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a951")]
                                        // "c001_a951" => Some(tig_algorithms::satisfiability::c001_a951::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a952")]
                                        // "c001_a952" => Some(tig_algorithms::satisfiability::c001_a952::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a953")]
                                        // "c001_a953" => Some(tig_algorithms::satisfiability::c001_a953::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a954")]
                                        // "c001_a954" => Some(tig_algorithms::satisfiability::c001_a954::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a955")]
                                        // "c001_a955" => Some(tig_algorithms::satisfiability::c001_a955::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a956")]
                                        // "c001_a956" => Some(tig_algorithms::satisfiability::c001_a956::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a957")]
                                        // "c001_a957" => Some(tig_algorithms::satisfiability::c001_a957::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a958")]
                                        // "c001_a958" => Some(tig_algorithms::satisfiability::c001_a958::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a959")]
                                        // "c001_a959" => Some(tig_algorithms::satisfiability::c001_a959::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a960")]
                                        // "c001_a960" => Some(tig_algorithms::satisfiability::c001_a960::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a961")]
                                        // "c001_a961" => Some(tig_algorithms::satisfiability::c001_a961::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a962")]
                                        // "c001_a962" => Some(tig_algorithms::satisfiability::c001_a962::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a963")]
                                        // "c001_a963" => Some(tig_algorithms::satisfiability::c001_a963::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a964")]
                                        // "c001_a964" => Some(tig_algorithms::satisfiability::c001_a964::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a965")]
                                        // "c001_a965" => Some(tig_algorithms::satisfiability::c001_a965::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a966")]
                                        // "c001_a966" => Some(tig_algorithms::satisfiability::c001_a966::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a967")]
                                        // "c001_a967" => Some(tig_algorithms::satisfiability::c001_a967::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a968")]
                                        // "c001_a968" => Some(tig_algorithms::satisfiability::c001_a968::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a969")]
                                        // "c001_a969" => Some(tig_algorithms::satisfiability::c001_a969::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a970")]
                                        // "c001_a970" => Some(tig_algorithms::satisfiability::c001_a970::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a971")]
                                        // "c001_a971" => Some(tig_algorithms::satisfiability::c001_a971::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a972")]
                                        // "c001_a972" => Some(tig_algorithms::satisfiability::c001_a972::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a973")]
                                        // "c001_a973" => Some(tig_algorithms::satisfiability::c001_a973::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a974")]
                                        // "c001_a974" => Some(tig_algorithms::satisfiability::c001_a974::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a975")]
                                        // "c001_a975" => Some(tig_algorithms::satisfiability::c001_a975::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a976")]
                                        // "c001_a976" => Some(tig_algorithms::satisfiability::c001_a976::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a977")]
                                        // "c001_a977" => Some(tig_algorithms::satisfiability::c001_a977::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a978")]
                                        // "c001_a978" => Some(tig_algorithms::satisfiability::c001_a978::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a979")]
                                        // "c001_a979" => Some(tig_algorithms::satisfiability::c001_a979::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a980")]
                                        // "c001_a980" => Some(tig_algorithms::satisfiability::c001_a980::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a981")]
                                        // "c001_a981" => Some(tig_algorithms::satisfiability::c001_a981::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a982")]
                                        // "c001_a982" => Some(tig_algorithms::satisfiability::c001_a982::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a983")]
                                        // "c001_a983" => Some(tig_algorithms::satisfiability::c001_a983::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a984")]
                                        // "c001_a984" => Some(tig_algorithms::satisfiability::c001_a984::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a985")]
                                        // "c001_a985" => Some(tig_algorithms::satisfiability::c001_a985::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a986")]
                                        // "c001_a986" => Some(tig_algorithms::satisfiability::c001_a986::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a987")]
                                        // "c001_a987" => Some(tig_algorithms::satisfiability::c001_a987::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a988")]
                                        // "c001_a988" => Some(tig_algorithms::satisfiability::c001_a988::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a989")]
                                        // "c001_a989" => Some(tig_algorithms::satisfiability::c001_a989::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a990")]
                                        // "c001_a990" => Some(tig_algorithms::satisfiability::c001_a990::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a991")]
                                        // "c001_a991" => Some(tig_algorithms::satisfiability::c001_a991::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a992")]
                                        // "c001_a992" => Some(tig_algorithms::satisfiability::c001_a992::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a993")]
                                        // "c001_a993" => Some(tig_algorithms::satisfiability::c001_a993::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a994")]
                                        // "c001_a994" => Some(tig_algorithms::satisfiability::c001_a994::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a995")]
                                        // "c001_a995" => Some(tig_algorithms::satisfiability::c001_a995::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a996")]
                                        // "c001_a996" => Some(tig_algorithms::satisfiability::c001_a996::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a997")]
                                        // "c001_a997" => Some(tig_algorithms::satisfiability::c001_a997::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a998")]
                                        // "c001_a998" => Some(tig_algorithms::satisfiability::c001_a998::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c001_a999")]
                                        // "c001_a999" => Some(tig_algorithms::satisfiability::c001_a999::solve_challenge as SolveChallengeFn),

                                        _ => Option::<SolveChallengeFn>::None
                                    }
                                {
                                    let seed = job.settings.calc_seed(nonce);
                                    let challenge = tig_challenges::satisfiability::Challenge::generate_instance_from_vec(seed, &job.settings.difficulty).unwrap();
                                    run_wasm = false;
                                    if let Ok(Some(solution)) = solve_challenge(&challenge) {
                                        run_wasm =
                                            challenge.verify_solution(&solution).is_ok();
                                    }
                                }
                            }
                            "c002" => {
                                type SolveChallengeFn = fn(&tig_challenges::vehicle_routing::Challenge) -> anyhow::Result<Option<tig_challenges::vehicle_routing::Solution>>;
                                if let Some(solve_challenge) =
                                    match job.settings.algorithm_id.as_str() {
                                        // #[cfg(feature = "c002_a001")]
                                        // "c002_a001" => Some(tig_algorithms::vehicle_routing::c002_a001::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a002")]
                                        // "c002_a002" => Some(tig_algorithms::vehicle_routing::c002_a002::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a003")]
                                        // "c002_a003" => Some(tig_algorithms::vehicle_routing::c002_a003::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a004")]
                                        // "c002_a004" => Some(tig_algorithms::vehicle_routing::c002_a004::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a005")]
                                        // "c002_a005" => Some(tig_algorithms::vehicle_routing::c002_a005::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a006")]
                                        // "c002_a006" => Some(tig_algorithms::vehicle_routing::c002_a006::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a007")]
                                        // "c002_a007" => Some(tig_algorithms::vehicle_routing::c002_a007::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a008")]
                                        // "c002_a008" => Some(tig_algorithms::vehicle_routing::c002_a008::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a009")]
                                        // "c002_a009" => Some(tig_algorithms::vehicle_routing::c002_a009::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a010")]
                                        // "c002_a010" => Some(tig_algorithms::vehicle_routing::c002_a010::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a011")]
                                        // "c002_a011" => Some(tig_algorithms::vehicle_routing::c002_a011::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a012")]
                                        // "c002_a012" => Some(tig_algorithms::vehicle_routing::c002_a012::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a013")]
                                        // "c002_a013" => Some(tig_algorithms::vehicle_routing::c002_a013::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a014")]
                                        // "c002_a014" => Some(tig_algorithms::vehicle_routing::c002_a014::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a015")]
                                        // "c002_a015" => Some(tig_algorithms::vehicle_routing::c002_a015::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a016")]
                                        // "c002_a016" => Some(tig_algorithms::vehicle_routing::c002_a016::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a017")]
                                        // "c002_a017" => Some(tig_algorithms::vehicle_routing::c002_a017::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a018")]
                                        // "c002_a018" => Some(tig_algorithms::vehicle_routing::c002_a018::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a019")]
                                        // "c002_a019" => Some(tig_algorithms::vehicle_routing::c002_a019::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a020")]
                                        // "c002_a020" => Some(tig_algorithms::vehicle_routing::c002_a020::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a021")]
                                        // "c002_a021" => Some(tig_algorithms::vehicle_routing::c002_a021::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a022")]
                                        // "c002_a022" => Some(tig_algorithms::vehicle_routing::c002_a022::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a023")]
                                        // "c002_a023" => Some(tig_algorithms::vehicle_routing::c002_a023::solve_challenge as SolveChallengeFn),
                                        
                                        #[cfg(feature = "vehicle_routing_cw_two_opt_ls")]
                                        "c002_a024" => Some(tig_algorithms::vehicle_routing::cw_two_opt_ls::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a025")]
                                        // "c002_a025" => Some(tig_algorithms::vehicle_routing::c002_a025::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a026")]
                                        // "c002_a026" => Some(tig_algorithms::vehicle_routing::c002_a026::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a027")]
                                        // "c002_a027" => Some(tig_algorithms::vehicle_routing::c002_a027::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a028")]
                                        // "c002_a028" => Some(tig_algorithms::vehicle_routing::c002_a028::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a029")]
                                        // "c002_a029" => Some(tig_algorithms::vehicle_routing::c002_a029::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a030")]
                                        // "c002_a030" => Some(tig_algorithms::vehicle_routing::c002_a030::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a031")]
                                        // "c002_a031" => Some(tig_algorithms::vehicle_routing::c002_a031::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a032")]
                                        // "c002_a032" => Some(tig_algorithms::vehicle_routing::c002_a032::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a033")]
                                        // "c002_a033" => Some(tig_algorithms::vehicle_routing::c002_a033::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a034")]
                                        // "c002_a034" => Some(tig_algorithms::vehicle_routing::c002_a034::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a035")]
                                        // "c002_a035" => Some(tig_algorithms::vehicle_routing::c002_a035::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a036")]
                                        // "c002_a036" => Some(tig_algorithms::vehicle_routing::c002_a036::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a037")]
                                        // "c002_a037" => Some(tig_algorithms::vehicle_routing::c002_a037::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a038")]
                                        // "c002_a038" => Some(tig_algorithms::vehicle_routing::c002_a038::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a039")]
                                        // "c002_a039" => Some(tig_algorithms::vehicle_routing::c002_a039::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a040")]
                                        // "c002_a040" => Some(tig_algorithms::vehicle_routing::c002_a040::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a041")]
                                        // "c002_a041" => Some(tig_algorithms::vehicle_routing::c002_a041::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a042")]
                                        // "c002_a042" => Some(tig_algorithms::vehicle_routing::c002_a042::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a043")]
                                        // "c002_a043" => Some(tig_algorithms::vehicle_routing::c002_a043::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a044")]
                                        // "c002_a044" => Some(tig_algorithms::vehicle_routing::c002_a044::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a045")]
                                        // "c002_a045" => Some(tig_algorithms::vehicle_routing::c002_a045::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a046")]
                                        // "c002_a046" => Some(tig_algorithms::vehicle_routing::c002_a046::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a047")]
                                        // "c002_a047" => Some(tig_algorithms::vehicle_routing::c002_a047::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a048")]
                                        // "c002_a048" => Some(tig_algorithms::vehicle_routing::c002_a048::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a049")]
                                        // "c002_a049" => Some(tig_algorithms::vehicle_routing::c002_a049::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a050")]
                                        // "c002_a050" => Some(tig_algorithms::vehicle_routing::c002_a050::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a051")]
                                        // "c002_a051" => Some(tig_algorithms::vehicle_routing::c002_a051::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a052")]
                                        // "c002_a052" => Some(tig_algorithms::vehicle_routing::c002_a052::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a053")]
                                        // "c002_a053" => Some(tig_algorithms::vehicle_routing::c002_a053::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a054")]
                                        // "c002_a054" => Some(tig_algorithms::vehicle_routing::c002_a054::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a055")]
                                        // "c002_a055" => Some(tig_algorithms::vehicle_routing::c002_a055::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a056")]
                                        // "c002_a056" => Some(tig_algorithms::vehicle_routing::c002_a056::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a057")]
                                        // "c002_a057" => Some(tig_algorithms::vehicle_routing::c002_a057::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a058")]
                                        // "c002_a058" => Some(tig_algorithms::vehicle_routing::c002_a058::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a059")]
                                        // "c002_a059" => Some(tig_algorithms::vehicle_routing::c002_a059::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a060")]
                                        // "c002_a060" => Some(tig_algorithms::vehicle_routing::c002_a060::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a061")]
                                        // "c002_a061" => Some(tig_algorithms::vehicle_routing::c002_a061::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a062")]
                                        // "c002_a062" => Some(tig_algorithms::vehicle_routing::c002_a062::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a063")]
                                        // "c002_a063" => Some(tig_algorithms::vehicle_routing::c002_a063::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a064")]
                                        // "c002_a064" => Some(tig_algorithms::vehicle_routing::c002_a064::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a065")]
                                        // "c002_a065" => Some(tig_algorithms::vehicle_routing::c002_a065::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a066")]
                                        // "c002_a066" => Some(tig_algorithms::vehicle_routing::c002_a066::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a067")]
                                        // "c002_a067" => Some(tig_algorithms::vehicle_routing::c002_a067::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a068")]
                                        // "c002_a068" => Some(tig_algorithms::vehicle_routing::c002_a068::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a069")]
                                        // "c002_a069" => Some(tig_algorithms::vehicle_routing::c002_a069::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a070")]
                                        // "c002_a070" => Some(tig_algorithms::vehicle_routing::c002_a070::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a071")]
                                        // "c002_a071" => Some(tig_algorithms::vehicle_routing::c002_a071::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a072")]
                                        // "c002_a072" => Some(tig_algorithms::vehicle_routing::c002_a072::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a073")]
                                        // "c002_a073" => Some(tig_algorithms::vehicle_routing::c002_a073::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a074")]
                                        // "c002_a074" => Some(tig_algorithms::vehicle_routing::c002_a074::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a075")]
                                        // "c002_a075" => Some(tig_algorithms::vehicle_routing::c002_a075::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a076")]
                                        // "c002_a076" => Some(tig_algorithms::vehicle_routing::c002_a076::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a077")]
                                        // "c002_a077" => Some(tig_algorithms::vehicle_routing::c002_a077::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a078")]
                                        // "c002_a078" => Some(tig_algorithms::vehicle_routing::c002_a078::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a079")]
                                        // "c002_a079" => Some(tig_algorithms::vehicle_routing::c002_a079::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a080")]
                                        // "c002_a080" => Some(tig_algorithms::vehicle_routing::c002_a080::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a081")]
                                        // "c002_a081" => Some(tig_algorithms::vehicle_routing::c002_a081::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a082")]
                                        // "c002_a082" => Some(tig_algorithms::vehicle_routing::c002_a082::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a083")]
                                        // "c002_a083" => Some(tig_algorithms::vehicle_routing::c002_a083::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a084")]
                                        // "c002_a084" => Some(tig_algorithms::vehicle_routing::c002_a084::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a085")]
                                        // "c002_a085" => Some(tig_algorithms::vehicle_routing::c002_a085::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a086")]
                                        // "c002_a086" => Some(tig_algorithms::vehicle_routing::c002_a086::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a087")]
                                        // "c002_a087" => Some(tig_algorithms::vehicle_routing::c002_a087::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a088")]
                                        // "c002_a088" => Some(tig_algorithms::vehicle_routing::c002_a088::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a089")]
                                        // "c002_a089" => Some(tig_algorithms::vehicle_routing::c002_a089::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a090")]
                                        // "c002_a090" => Some(tig_algorithms::vehicle_routing::c002_a090::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a091")]
                                        // "c002_a091" => Some(tig_algorithms::vehicle_routing::c002_a091::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a092")]
                                        // "c002_a092" => Some(tig_algorithms::vehicle_routing::c002_a092::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a093")]
                                        // "c002_a093" => Some(tig_algorithms::vehicle_routing::c002_a093::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a094")]
                                        // "c002_a094" => Some(tig_algorithms::vehicle_routing::c002_a094::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a095")]
                                        // "c002_a095" => Some(tig_algorithms::vehicle_routing::c002_a095::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a096")]
                                        // "c002_a096" => Some(tig_algorithms::vehicle_routing::c002_a096::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a097")]
                                        // "c002_a097" => Some(tig_algorithms::vehicle_routing::c002_a097::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a098")]
                                        // "c002_a098" => Some(tig_algorithms::vehicle_routing::c002_a098::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a099")]
                                        // "c002_a099" => Some(tig_algorithms::vehicle_routing::c002_a099::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a100")]
                                        // "c002_a100" => Some(tig_algorithms::vehicle_routing::c002_a100::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a101")]
                                        // "c002_a101" => Some(tig_algorithms::vehicle_routing::c002_a101::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a102")]
                                        // "c002_a102" => Some(tig_algorithms::vehicle_routing::c002_a102::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a103")]
                                        // "c002_a103" => Some(tig_algorithms::vehicle_routing::c002_a103::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a104")]
                                        // "c002_a104" => Some(tig_algorithms::vehicle_routing::c002_a104::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a105")]
                                        // "c002_a105" => Some(tig_algorithms::vehicle_routing::c002_a105::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a106")]
                                        // "c002_a106" => Some(tig_algorithms::vehicle_routing::c002_a106::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a107")]
                                        // "c002_a107" => Some(tig_algorithms::vehicle_routing::c002_a107::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a108")]
                                        // "c002_a108" => Some(tig_algorithms::vehicle_routing::c002_a108::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a109")]
                                        // "c002_a109" => Some(tig_algorithms::vehicle_routing::c002_a109::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a110")]
                                        // "c002_a110" => Some(tig_algorithms::vehicle_routing::c002_a110::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a111")]
                                        // "c002_a111" => Some(tig_algorithms::vehicle_routing::c002_a111::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a112")]
                                        // "c002_a112" => Some(tig_algorithms::vehicle_routing::c002_a112::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a113")]
                                        // "c002_a113" => Some(tig_algorithms::vehicle_routing::c002_a113::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a114")]
                                        // "c002_a114" => Some(tig_algorithms::vehicle_routing::c002_a114::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a115")]
                                        // "c002_a115" => Some(tig_algorithms::vehicle_routing::c002_a115::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a116")]
                                        // "c002_a116" => Some(tig_algorithms::vehicle_routing::c002_a116::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a117")]
                                        // "c002_a117" => Some(tig_algorithms::vehicle_routing::c002_a117::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a118")]
                                        // "c002_a118" => Some(tig_algorithms::vehicle_routing::c002_a118::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a119")]
                                        // "c002_a119" => Some(tig_algorithms::vehicle_routing::c002_a119::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a120")]
                                        // "c002_a120" => Some(tig_algorithms::vehicle_routing::c002_a120::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a121")]
                                        // "c002_a121" => Some(tig_algorithms::vehicle_routing::c002_a121::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a122")]
                                        // "c002_a122" => Some(tig_algorithms::vehicle_routing::c002_a122::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a123")]
                                        // "c002_a123" => Some(tig_algorithms::vehicle_routing::c002_a123::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a124")]
                                        // "c002_a124" => Some(tig_algorithms::vehicle_routing::c002_a124::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a125")]
                                        // "c002_a125" => Some(tig_algorithms::vehicle_routing::c002_a125::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a126")]
                                        // "c002_a126" => Some(tig_algorithms::vehicle_routing::c002_a126::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a127")]
                                        // "c002_a127" => Some(tig_algorithms::vehicle_routing::c002_a127::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a128")]
                                        // "c002_a128" => Some(tig_algorithms::vehicle_routing::c002_a128::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a129")]
                                        // "c002_a129" => Some(tig_algorithms::vehicle_routing::c002_a129::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a130")]
                                        // "c002_a130" => Some(tig_algorithms::vehicle_routing::c002_a130::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a131")]
                                        // "c002_a131" => Some(tig_algorithms::vehicle_routing::c002_a131::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a132")]
                                        // "c002_a132" => Some(tig_algorithms::vehicle_routing::c002_a132::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a133")]
                                        // "c002_a133" => Some(tig_algorithms::vehicle_routing::c002_a133::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a134")]
                                        // "c002_a134" => Some(tig_algorithms::vehicle_routing::c002_a134::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a135")]
                                        // "c002_a135" => Some(tig_algorithms::vehicle_routing::c002_a135::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a136")]
                                        // "c002_a136" => Some(tig_algorithms::vehicle_routing::c002_a136::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a137")]
                                        // "c002_a137" => Some(tig_algorithms::vehicle_routing::c002_a137::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a138")]
                                        // "c002_a138" => Some(tig_algorithms::vehicle_routing::c002_a138::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a139")]
                                        // "c002_a139" => Some(tig_algorithms::vehicle_routing::c002_a139::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a140")]
                                        // "c002_a140" => Some(tig_algorithms::vehicle_routing::c002_a140::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a141")]
                                        // "c002_a141" => Some(tig_algorithms::vehicle_routing::c002_a141::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a142")]
                                        // "c002_a142" => Some(tig_algorithms::vehicle_routing::c002_a142::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a143")]
                                        // "c002_a143" => Some(tig_algorithms::vehicle_routing::c002_a143::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a144")]
                                        // "c002_a144" => Some(tig_algorithms::vehicle_routing::c002_a144::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a145")]
                                        // "c002_a145" => Some(tig_algorithms::vehicle_routing::c002_a145::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a146")]
                                        // "c002_a146" => Some(tig_algorithms::vehicle_routing::c002_a146::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a147")]
                                        // "c002_a147" => Some(tig_algorithms::vehicle_routing::c002_a147::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a148")]
                                        // "c002_a148" => Some(tig_algorithms::vehicle_routing::c002_a148::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a149")]
                                        // "c002_a149" => Some(tig_algorithms::vehicle_routing::c002_a149::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a150")]
                                        // "c002_a150" => Some(tig_algorithms::vehicle_routing::c002_a150::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a151")]
                                        // "c002_a151" => Some(tig_algorithms::vehicle_routing::c002_a151::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a152")]
                                        // "c002_a152" => Some(tig_algorithms::vehicle_routing::c002_a152::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a153")]
                                        // "c002_a153" => Some(tig_algorithms::vehicle_routing::c002_a153::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a154")]
                                        // "c002_a154" => Some(tig_algorithms::vehicle_routing::c002_a154::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a155")]
                                        // "c002_a155" => Some(tig_algorithms::vehicle_routing::c002_a155::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a156")]
                                        // "c002_a156" => Some(tig_algorithms::vehicle_routing::c002_a156::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a157")]
                                        // "c002_a157" => Some(tig_algorithms::vehicle_routing::c002_a157::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a158")]
                                        // "c002_a158" => Some(tig_algorithms::vehicle_routing::c002_a158::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a159")]
                                        // "c002_a159" => Some(tig_algorithms::vehicle_routing::c002_a159::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a160")]
                                        // "c002_a160" => Some(tig_algorithms::vehicle_routing::c002_a160::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a161")]
                                        // "c002_a161" => Some(tig_algorithms::vehicle_routing::c002_a161::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a162")]
                                        // "c002_a162" => Some(tig_algorithms::vehicle_routing::c002_a162::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a163")]
                                        // "c002_a163" => Some(tig_algorithms::vehicle_routing::c002_a163::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a164")]
                                        // "c002_a164" => Some(tig_algorithms::vehicle_routing::c002_a164::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a165")]
                                        // "c002_a165" => Some(tig_algorithms::vehicle_routing::c002_a165::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a166")]
                                        // "c002_a166" => Some(tig_algorithms::vehicle_routing::c002_a166::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a167")]
                                        // "c002_a167" => Some(tig_algorithms::vehicle_routing::c002_a167::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a168")]
                                        // "c002_a168" => Some(tig_algorithms::vehicle_routing::c002_a168::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a169")]
                                        // "c002_a169" => Some(tig_algorithms::vehicle_routing::c002_a169::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a170")]
                                        // "c002_a170" => Some(tig_algorithms::vehicle_routing::c002_a170::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a171")]
                                        // "c002_a171" => Some(tig_algorithms::vehicle_routing::c002_a171::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a172")]
                                        // "c002_a172" => Some(tig_algorithms::vehicle_routing::c002_a172::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a173")]
                                        // "c002_a173" => Some(tig_algorithms::vehicle_routing::c002_a173::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a174")]
                                        // "c002_a174" => Some(tig_algorithms::vehicle_routing::c002_a174::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a175")]
                                        // "c002_a175" => Some(tig_algorithms::vehicle_routing::c002_a175::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a176")]
                                        // "c002_a176" => Some(tig_algorithms::vehicle_routing::c002_a176::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a177")]
                                        // "c002_a177" => Some(tig_algorithms::vehicle_routing::c002_a177::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a178")]
                                        // "c002_a178" => Some(tig_algorithms::vehicle_routing::c002_a178::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a179")]
                                        // "c002_a179" => Some(tig_algorithms::vehicle_routing::c002_a179::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a180")]
                                        // "c002_a180" => Some(tig_algorithms::vehicle_routing::c002_a180::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a181")]
                                        // "c002_a181" => Some(tig_algorithms::vehicle_routing::c002_a181::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a182")]
                                        // "c002_a182" => Some(tig_algorithms::vehicle_routing::c002_a182::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a183")]
                                        // "c002_a183" => Some(tig_algorithms::vehicle_routing::c002_a183::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a184")]
                                        // "c002_a184" => Some(tig_algorithms::vehicle_routing::c002_a184::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a185")]
                                        // "c002_a185" => Some(tig_algorithms::vehicle_routing::c002_a185::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a186")]
                                        // "c002_a186" => Some(tig_algorithms::vehicle_routing::c002_a186::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a187")]
                                        // "c002_a187" => Some(tig_algorithms::vehicle_routing::c002_a187::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a188")]
                                        // "c002_a188" => Some(tig_algorithms::vehicle_routing::c002_a188::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a189")]
                                        // "c002_a189" => Some(tig_algorithms::vehicle_routing::c002_a189::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a190")]
                                        // "c002_a190" => Some(tig_algorithms::vehicle_routing::c002_a190::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a191")]
                                        // "c002_a191" => Some(tig_algorithms::vehicle_routing::c002_a191::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a192")]
                                        // "c002_a192" => Some(tig_algorithms::vehicle_routing::c002_a192::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a193")]
                                        // "c002_a193" => Some(tig_algorithms::vehicle_routing::c002_a193::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a194")]
                                        // "c002_a194" => Some(tig_algorithms::vehicle_routing::c002_a194::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a195")]
                                        // "c002_a195" => Some(tig_algorithms::vehicle_routing::c002_a195::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a196")]
                                        // "c002_a196" => Some(tig_algorithms::vehicle_routing::c002_a196::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a197")]
                                        // "c002_a197" => Some(tig_algorithms::vehicle_routing::c002_a197::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a198")]
                                        // "c002_a198" => Some(tig_algorithms::vehicle_routing::c002_a198::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a199")]
                                        // "c002_a199" => Some(tig_algorithms::vehicle_routing::c002_a199::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a200")]
                                        // "c002_a200" => Some(tig_algorithms::vehicle_routing::c002_a200::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a201")]
                                        // "c002_a201" => Some(tig_algorithms::vehicle_routing::c002_a201::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a202")]
                                        // "c002_a202" => Some(tig_algorithms::vehicle_routing::c002_a202::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a203")]
                                        // "c002_a203" => Some(tig_algorithms::vehicle_routing::c002_a203::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a204")]
                                        // "c002_a204" => Some(tig_algorithms::vehicle_routing::c002_a204::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a205")]
                                        // "c002_a205" => Some(tig_algorithms::vehicle_routing::c002_a205::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a206")]
                                        // "c002_a206" => Some(tig_algorithms::vehicle_routing::c002_a206::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a207")]
                                        // "c002_a207" => Some(tig_algorithms::vehicle_routing::c002_a207::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a208")]
                                        // "c002_a208" => Some(tig_algorithms::vehicle_routing::c002_a208::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a209")]
                                        // "c002_a209" => Some(tig_algorithms::vehicle_routing::c002_a209::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a210")]
                                        // "c002_a210" => Some(tig_algorithms::vehicle_routing::c002_a210::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a211")]
                                        // "c002_a211" => Some(tig_algorithms::vehicle_routing::c002_a211::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a212")]
                                        // "c002_a212" => Some(tig_algorithms::vehicle_routing::c002_a212::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a213")]
                                        // "c002_a213" => Some(tig_algorithms::vehicle_routing::c002_a213::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a214")]
                                        // "c002_a214" => Some(tig_algorithms::vehicle_routing::c002_a214::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a215")]
                                        // "c002_a215" => Some(tig_algorithms::vehicle_routing::c002_a215::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a216")]
                                        // "c002_a216" => Some(tig_algorithms::vehicle_routing::c002_a216::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a217")]
                                        // "c002_a217" => Some(tig_algorithms::vehicle_routing::c002_a217::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a218")]
                                        // "c002_a218" => Some(tig_algorithms::vehicle_routing::c002_a218::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a219")]
                                        // "c002_a219" => Some(tig_algorithms::vehicle_routing::c002_a219::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a220")]
                                        // "c002_a220" => Some(tig_algorithms::vehicle_routing::c002_a220::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a221")]
                                        // "c002_a221" => Some(tig_algorithms::vehicle_routing::c002_a221::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a222")]
                                        // "c002_a222" => Some(tig_algorithms::vehicle_routing::c002_a222::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a223")]
                                        // "c002_a223" => Some(tig_algorithms::vehicle_routing::c002_a223::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a224")]
                                        // "c002_a224" => Some(tig_algorithms::vehicle_routing::c002_a224::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a225")]
                                        // "c002_a225" => Some(tig_algorithms::vehicle_routing::c002_a225::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a226")]
                                        // "c002_a226" => Some(tig_algorithms::vehicle_routing::c002_a226::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a227")]
                                        // "c002_a227" => Some(tig_algorithms::vehicle_routing::c002_a227::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a228")]
                                        // "c002_a228" => Some(tig_algorithms::vehicle_routing::c002_a228::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a229")]
                                        // "c002_a229" => Some(tig_algorithms::vehicle_routing::c002_a229::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a230")]
                                        // "c002_a230" => Some(tig_algorithms::vehicle_routing::c002_a230::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a231")]
                                        // "c002_a231" => Some(tig_algorithms::vehicle_routing::c002_a231::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a232")]
                                        // "c002_a232" => Some(tig_algorithms::vehicle_routing::c002_a232::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a233")]
                                        // "c002_a233" => Some(tig_algorithms::vehicle_routing::c002_a233::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a234")]
                                        // "c002_a234" => Some(tig_algorithms::vehicle_routing::c002_a234::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a235")]
                                        // "c002_a235" => Some(tig_algorithms::vehicle_routing::c002_a235::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a236")]
                                        // "c002_a236" => Some(tig_algorithms::vehicle_routing::c002_a236::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a237")]
                                        // "c002_a237" => Some(tig_algorithms::vehicle_routing::c002_a237::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a238")]
                                        // "c002_a238" => Some(tig_algorithms::vehicle_routing::c002_a238::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a239")]
                                        // "c002_a239" => Some(tig_algorithms::vehicle_routing::c002_a239::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a240")]
                                        // "c002_a240" => Some(tig_algorithms::vehicle_routing::c002_a240::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a241")]
                                        // "c002_a241" => Some(tig_algorithms::vehicle_routing::c002_a241::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a242")]
                                        // "c002_a242" => Some(tig_algorithms::vehicle_routing::c002_a242::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a243")]
                                        // "c002_a243" => Some(tig_algorithms::vehicle_routing::c002_a243::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a244")]
                                        // "c002_a244" => Some(tig_algorithms::vehicle_routing::c002_a244::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a245")]
                                        // "c002_a245" => Some(tig_algorithms::vehicle_routing::c002_a245::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a246")]
                                        // "c002_a246" => Some(tig_algorithms::vehicle_routing::c002_a246::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a247")]
                                        // "c002_a247" => Some(tig_algorithms::vehicle_routing::c002_a247::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a248")]
                                        // "c002_a248" => Some(tig_algorithms::vehicle_routing::c002_a248::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a249")]
                                        // "c002_a249" => Some(tig_algorithms::vehicle_routing::c002_a249::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a250")]
                                        // "c002_a250" => Some(tig_algorithms::vehicle_routing::c002_a250::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a251")]
                                        // "c002_a251" => Some(tig_algorithms::vehicle_routing::c002_a251::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a252")]
                                        // "c002_a252" => Some(tig_algorithms::vehicle_routing::c002_a252::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a253")]
                                        // "c002_a253" => Some(tig_algorithms::vehicle_routing::c002_a253::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a254")]
                                        // "c002_a254" => Some(tig_algorithms::vehicle_routing::c002_a254::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a255")]
                                        // "c002_a255" => Some(tig_algorithms::vehicle_routing::c002_a255::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a256")]
                                        // "c002_a256" => Some(tig_algorithms::vehicle_routing::c002_a256::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a257")]
                                        // "c002_a257" => Some(tig_algorithms::vehicle_routing::c002_a257::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a258")]
                                        // "c002_a258" => Some(tig_algorithms::vehicle_routing::c002_a258::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a259")]
                                        // "c002_a259" => Some(tig_algorithms::vehicle_routing::c002_a259::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a260")]
                                        // "c002_a260" => Some(tig_algorithms::vehicle_routing::c002_a260::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a261")]
                                        // "c002_a261" => Some(tig_algorithms::vehicle_routing::c002_a261::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a262")]
                                        // "c002_a262" => Some(tig_algorithms::vehicle_routing::c002_a262::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a263")]
                                        // "c002_a263" => Some(tig_algorithms::vehicle_routing::c002_a263::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a264")]
                                        // "c002_a264" => Some(tig_algorithms::vehicle_routing::c002_a264::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a265")]
                                        // "c002_a265" => Some(tig_algorithms::vehicle_routing::c002_a265::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a266")]
                                        // "c002_a266" => Some(tig_algorithms::vehicle_routing::c002_a266::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a267")]
                                        // "c002_a267" => Some(tig_algorithms::vehicle_routing::c002_a267::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a268")]
                                        // "c002_a268" => Some(tig_algorithms::vehicle_routing::c002_a268::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a269")]
                                        // "c002_a269" => Some(tig_algorithms::vehicle_routing::c002_a269::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a270")]
                                        // "c002_a270" => Some(tig_algorithms::vehicle_routing::c002_a270::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a271")]
                                        // "c002_a271" => Some(tig_algorithms::vehicle_routing::c002_a271::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a272")]
                                        // "c002_a272" => Some(tig_algorithms::vehicle_routing::c002_a272::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a273")]
                                        // "c002_a273" => Some(tig_algorithms::vehicle_routing::c002_a273::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a274")]
                                        // "c002_a274" => Some(tig_algorithms::vehicle_routing::c002_a274::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a275")]
                                        // "c002_a275" => Some(tig_algorithms::vehicle_routing::c002_a275::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a276")]
                                        // "c002_a276" => Some(tig_algorithms::vehicle_routing::c002_a276::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a277")]
                                        // "c002_a277" => Some(tig_algorithms::vehicle_routing::c002_a277::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a278")]
                                        // "c002_a278" => Some(tig_algorithms::vehicle_routing::c002_a278::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a279")]
                                        // "c002_a279" => Some(tig_algorithms::vehicle_routing::c002_a279::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a280")]
                                        // "c002_a280" => Some(tig_algorithms::vehicle_routing::c002_a280::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a281")]
                                        // "c002_a281" => Some(tig_algorithms::vehicle_routing::c002_a281::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a282")]
                                        // "c002_a282" => Some(tig_algorithms::vehicle_routing::c002_a282::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a283")]
                                        // "c002_a283" => Some(tig_algorithms::vehicle_routing::c002_a283::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a284")]
                                        // "c002_a284" => Some(tig_algorithms::vehicle_routing::c002_a284::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a285")]
                                        // "c002_a285" => Some(tig_algorithms::vehicle_routing::c002_a285::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a286")]
                                        // "c002_a286" => Some(tig_algorithms::vehicle_routing::c002_a286::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a287")]
                                        // "c002_a287" => Some(tig_algorithms::vehicle_routing::c002_a287::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a288")]
                                        // "c002_a288" => Some(tig_algorithms::vehicle_routing::c002_a288::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a289")]
                                        // "c002_a289" => Some(tig_algorithms::vehicle_routing::c002_a289::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a290")]
                                        // "c002_a290" => Some(tig_algorithms::vehicle_routing::c002_a290::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a291")]
                                        // "c002_a291" => Some(tig_algorithms::vehicle_routing::c002_a291::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a292")]
                                        // "c002_a292" => Some(tig_algorithms::vehicle_routing::c002_a292::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a293")]
                                        // "c002_a293" => Some(tig_algorithms::vehicle_routing::c002_a293::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a294")]
                                        // "c002_a294" => Some(tig_algorithms::vehicle_routing::c002_a294::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a295")]
                                        // "c002_a295" => Some(tig_algorithms::vehicle_routing::c002_a295::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a296")]
                                        // "c002_a296" => Some(tig_algorithms::vehicle_routing::c002_a296::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a297")]
                                        // "c002_a297" => Some(tig_algorithms::vehicle_routing::c002_a297::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a298")]
                                        // "c002_a298" => Some(tig_algorithms::vehicle_routing::c002_a298::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a299")]
                                        // "c002_a299" => Some(tig_algorithms::vehicle_routing::c002_a299::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a300")]
                                        // "c002_a300" => Some(tig_algorithms::vehicle_routing::c002_a300::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a301")]
                                        // "c002_a301" => Some(tig_algorithms::vehicle_routing::c002_a301::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a302")]
                                        // "c002_a302" => Some(tig_algorithms::vehicle_routing::c002_a302::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a303")]
                                        // "c002_a303" => Some(tig_algorithms::vehicle_routing::c002_a303::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a304")]
                                        // "c002_a304" => Some(tig_algorithms::vehicle_routing::c002_a304::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a305")]
                                        // "c002_a305" => Some(tig_algorithms::vehicle_routing::c002_a305::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a306")]
                                        // "c002_a306" => Some(tig_algorithms::vehicle_routing::c002_a306::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a307")]
                                        // "c002_a307" => Some(tig_algorithms::vehicle_routing::c002_a307::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a308")]
                                        // "c002_a308" => Some(tig_algorithms::vehicle_routing::c002_a308::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a309")]
                                        // "c002_a309" => Some(tig_algorithms::vehicle_routing::c002_a309::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a310")]
                                        // "c002_a310" => Some(tig_algorithms::vehicle_routing::c002_a310::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a311")]
                                        // "c002_a311" => Some(tig_algorithms::vehicle_routing::c002_a311::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a312")]
                                        // "c002_a312" => Some(tig_algorithms::vehicle_routing::c002_a312::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a313")]
                                        // "c002_a313" => Some(tig_algorithms::vehicle_routing::c002_a313::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a314")]
                                        // "c002_a314" => Some(tig_algorithms::vehicle_routing::c002_a314::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a315")]
                                        // "c002_a315" => Some(tig_algorithms::vehicle_routing::c002_a315::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a316")]
                                        // "c002_a316" => Some(tig_algorithms::vehicle_routing::c002_a316::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a317")]
                                        // "c002_a317" => Some(tig_algorithms::vehicle_routing::c002_a317::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a318")]
                                        // "c002_a318" => Some(tig_algorithms::vehicle_routing::c002_a318::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a319")]
                                        // "c002_a319" => Some(tig_algorithms::vehicle_routing::c002_a319::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a320")]
                                        // "c002_a320" => Some(tig_algorithms::vehicle_routing::c002_a320::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a321")]
                                        // "c002_a321" => Some(tig_algorithms::vehicle_routing::c002_a321::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a322")]
                                        // "c002_a322" => Some(tig_algorithms::vehicle_routing::c002_a322::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a323")]
                                        // "c002_a323" => Some(tig_algorithms::vehicle_routing::c002_a323::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a324")]
                                        // "c002_a324" => Some(tig_algorithms::vehicle_routing::c002_a324::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a325")]
                                        // "c002_a325" => Some(tig_algorithms::vehicle_routing::c002_a325::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a326")]
                                        // "c002_a326" => Some(tig_algorithms::vehicle_routing::c002_a326::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a327")]
                                        // "c002_a327" => Some(tig_algorithms::vehicle_routing::c002_a327::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a328")]
                                        // "c002_a328" => Some(tig_algorithms::vehicle_routing::c002_a328::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a329")]
                                        // "c002_a329" => Some(tig_algorithms::vehicle_routing::c002_a329::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a330")]
                                        // "c002_a330" => Some(tig_algorithms::vehicle_routing::c002_a330::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a331")]
                                        // "c002_a331" => Some(tig_algorithms::vehicle_routing::c002_a331::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a332")]
                                        // "c002_a332" => Some(tig_algorithms::vehicle_routing::c002_a332::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a333")]
                                        // "c002_a333" => Some(tig_algorithms::vehicle_routing::c002_a333::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a334")]
                                        // "c002_a334" => Some(tig_algorithms::vehicle_routing::c002_a334::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a335")]
                                        // "c002_a335" => Some(tig_algorithms::vehicle_routing::c002_a335::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a336")]
                                        // "c002_a336" => Some(tig_algorithms::vehicle_routing::c002_a336::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a337")]
                                        // "c002_a337" => Some(tig_algorithms::vehicle_routing::c002_a337::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a338")]
                                        // "c002_a338" => Some(tig_algorithms::vehicle_routing::c002_a338::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a339")]
                                        // "c002_a339" => Some(tig_algorithms::vehicle_routing::c002_a339::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a340")]
                                        // "c002_a340" => Some(tig_algorithms::vehicle_routing::c002_a340::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a341")]
                                        // "c002_a341" => Some(tig_algorithms::vehicle_routing::c002_a341::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a342")]
                                        // "c002_a342" => Some(tig_algorithms::vehicle_routing::c002_a342::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a343")]
                                        // "c002_a343" => Some(tig_algorithms::vehicle_routing::c002_a343::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a344")]
                                        // "c002_a344" => Some(tig_algorithms::vehicle_routing::c002_a344::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a345")]
                                        // "c002_a345" => Some(tig_algorithms::vehicle_routing::c002_a345::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a346")]
                                        // "c002_a346" => Some(tig_algorithms::vehicle_routing::c002_a346::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a347")]
                                        // "c002_a347" => Some(tig_algorithms::vehicle_routing::c002_a347::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a348")]
                                        // "c002_a348" => Some(tig_algorithms::vehicle_routing::c002_a348::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a349")]
                                        // "c002_a349" => Some(tig_algorithms::vehicle_routing::c002_a349::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a350")]
                                        // "c002_a350" => Some(tig_algorithms::vehicle_routing::c002_a350::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a351")]
                                        // "c002_a351" => Some(tig_algorithms::vehicle_routing::c002_a351::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a352")]
                                        // "c002_a352" => Some(tig_algorithms::vehicle_routing::c002_a352::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a353")]
                                        // "c002_a353" => Some(tig_algorithms::vehicle_routing::c002_a353::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a354")]
                                        // "c002_a354" => Some(tig_algorithms::vehicle_routing::c002_a354::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a355")]
                                        // "c002_a355" => Some(tig_algorithms::vehicle_routing::c002_a355::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a356")]
                                        // "c002_a356" => Some(tig_algorithms::vehicle_routing::c002_a356::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a357")]
                                        // "c002_a357" => Some(tig_algorithms::vehicle_routing::c002_a357::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a358")]
                                        // "c002_a358" => Some(tig_algorithms::vehicle_routing::c002_a358::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a359")]
                                        // "c002_a359" => Some(tig_algorithms::vehicle_routing::c002_a359::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a360")]
                                        // "c002_a360" => Some(tig_algorithms::vehicle_routing::c002_a360::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a361")]
                                        // "c002_a361" => Some(tig_algorithms::vehicle_routing::c002_a361::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a362")]
                                        // "c002_a362" => Some(tig_algorithms::vehicle_routing::c002_a362::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a363")]
                                        // "c002_a363" => Some(tig_algorithms::vehicle_routing::c002_a363::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a364")]
                                        // "c002_a364" => Some(tig_algorithms::vehicle_routing::c002_a364::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a365")]
                                        // "c002_a365" => Some(tig_algorithms::vehicle_routing::c002_a365::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a366")]
                                        // "c002_a366" => Some(tig_algorithms::vehicle_routing::c002_a366::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a367")]
                                        // "c002_a367" => Some(tig_algorithms::vehicle_routing::c002_a367::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a368")]
                                        // "c002_a368" => Some(tig_algorithms::vehicle_routing::c002_a368::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a369")]
                                        // "c002_a369" => Some(tig_algorithms::vehicle_routing::c002_a369::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a370")]
                                        // "c002_a370" => Some(tig_algorithms::vehicle_routing::c002_a370::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a371")]
                                        // "c002_a371" => Some(tig_algorithms::vehicle_routing::c002_a371::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a372")]
                                        // "c002_a372" => Some(tig_algorithms::vehicle_routing::c002_a372::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a373")]
                                        // "c002_a373" => Some(tig_algorithms::vehicle_routing::c002_a373::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a374")]
                                        // "c002_a374" => Some(tig_algorithms::vehicle_routing::c002_a374::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a375")]
                                        // "c002_a375" => Some(tig_algorithms::vehicle_routing::c002_a375::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a376")]
                                        // "c002_a376" => Some(tig_algorithms::vehicle_routing::c002_a376::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a377")]
                                        // "c002_a377" => Some(tig_algorithms::vehicle_routing::c002_a377::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a378")]
                                        // "c002_a378" => Some(tig_algorithms::vehicle_routing::c002_a378::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a379")]
                                        // "c002_a379" => Some(tig_algorithms::vehicle_routing::c002_a379::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a380")]
                                        // "c002_a380" => Some(tig_algorithms::vehicle_routing::c002_a380::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a381")]
                                        // "c002_a381" => Some(tig_algorithms::vehicle_routing::c002_a381::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a382")]
                                        // "c002_a382" => Some(tig_algorithms::vehicle_routing::c002_a382::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a383")]
                                        // "c002_a383" => Some(tig_algorithms::vehicle_routing::c002_a383::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a384")]
                                        // "c002_a384" => Some(tig_algorithms::vehicle_routing::c002_a384::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a385")]
                                        // "c002_a385" => Some(tig_algorithms::vehicle_routing::c002_a385::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a386")]
                                        // "c002_a386" => Some(tig_algorithms::vehicle_routing::c002_a386::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a387")]
                                        // "c002_a387" => Some(tig_algorithms::vehicle_routing::c002_a387::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a388")]
                                        // "c002_a388" => Some(tig_algorithms::vehicle_routing::c002_a388::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a389")]
                                        // "c002_a389" => Some(tig_algorithms::vehicle_routing::c002_a389::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a390")]
                                        // "c002_a390" => Some(tig_algorithms::vehicle_routing::c002_a390::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a391")]
                                        // "c002_a391" => Some(tig_algorithms::vehicle_routing::c002_a391::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a392")]
                                        // "c002_a392" => Some(tig_algorithms::vehicle_routing::c002_a392::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a393")]
                                        // "c002_a393" => Some(tig_algorithms::vehicle_routing::c002_a393::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a394")]
                                        // "c002_a394" => Some(tig_algorithms::vehicle_routing::c002_a394::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a395")]
                                        // "c002_a395" => Some(tig_algorithms::vehicle_routing::c002_a395::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a396")]
                                        // "c002_a396" => Some(tig_algorithms::vehicle_routing::c002_a396::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a397")]
                                        // "c002_a397" => Some(tig_algorithms::vehicle_routing::c002_a397::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a398")]
                                        // "c002_a398" => Some(tig_algorithms::vehicle_routing::c002_a398::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a399")]
                                        // "c002_a399" => Some(tig_algorithms::vehicle_routing::c002_a399::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a400")]
                                        // "c002_a400" => Some(tig_algorithms::vehicle_routing::c002_a400::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a401")]
                                        // "c002_a401" => Some(tig_algorithms::vehicle_routing::c002_a401::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a402")]
                                        // "c002_a402" => Some(tig_algorithms::vehicle_routing::c002_a402::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a403")]
                                        // "c002_a403" => Some(tig_algorithms::vehicle_routing::c002_a403::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a404")]
                                        // "c002_a404" => Some(tig_algorithms::vehicle_routing::c002_a404::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a405")]
                                        // "c002_a405" => Some(tig_algorithms::vehicle_routing::c002_a405::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a406")]
                                        // "c002_a406" => Some(tig_algorithms::vehicle_routing::c002_a406::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a407")]
                                        // "c002_a407" => Some(tig_algorithms::vehicle_routing::c002_a407::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a408")]
                                        // "c002_a408" => Some(tig_algorithms::vehicle_routing::c002_a408::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a409")]
                                        // "c002_a409" => Some(tig_algorithms::vehicle_routing::c002_a409::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a410")]
                                        // "c002_a410" => Some(tig_algorithms::vehicle_routing::c002_a410::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a411")]
                                        // "c002_a411" => Some(tig_algorithms::vehicle_routing::c002_a411::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a412")]
                                        // "c002_a412" => Some(tig_algorithms::vehicle_routing::c002_a412::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a413")]
                                        // "c002_a413" => Some(tig_algorithms::vehicle_routing::c002_a413::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a414")]
                                        // "c002_a414" => Some(tig_algorithms::vehicle_routing::c002_a414::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a415")]
                                        // "c002_a415" => Some(tig_algorithms::vehicle_routing::c002_a415::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a416")]
                                        // "c002_a416" => Some(tig_algorithms::vehicle_routing::c002_a416::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a417")]
                                        // "c002_a417" => Some(tig_algorithms::vehicle_routing::c002_a417::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a418")]
                                        // "c002_a418" => Some(tig_algorithms::vehicle_routing::c002_a418::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a419")]
                                        // "c002_a419" => Some(tig_algorithms::vehicle_routing::c002_a419::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a420")]
                                        // "c002_a420" => Some(tig_algorithms::vehicle_routing::c002_a420::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a421")]
                                        // "c002_a421" => Some(tig_algorithms::vehicle_routing::c002_a421::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a422")]
                                        // "c002_a422" => Some(tig_algorithms::vehicle_routing::c002_a422::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a423")]
                                        // "c002_a423" => Some(tig_algorithms::vehicle_routing::c002_a423::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a424")]
                                        // "c002_a424" => Some(tig_algorithms::vehicle_routing::c002_a424::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a425")]
                                        // "c002_a425" => Some(tig_algorithms::vehicle_routing::c002_a425::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a426")]
                                        // "c002_a426" => Some(tig_algorithms::vehicle_routing::c002_a426::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a427")]
                                        // "c002_a427" => Some(tig_algorithms::vehicle_routing::c002_a427::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a428")]
                                        // "c002_a428" => Some(tig_algorithms::vehicle_routing::c002_a428::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a429")]
                                        // "c002_a429" => Some(tig_algorithms::vehicle_routing::c002_a429::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a430")]
                                        // "c002_a430" => Some(tig_algorithms::vehicle_routing::c002_a430::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a431")]
                                        // "c002_a431" => Some(tig_algorithms::vehicle_routing::c002_a431::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a432")]
                                        // "c002_a432" => Some(tig_algorithms::vehicle_routing::c002_a432::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a433")]
                                        // "c002_a433" => Some(tig_algorithms::vehicle_routing::c002_a433::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a434")]
                                        // "c002_a434" => Some(tig_algorithms::vehicle_routing::c002_a434::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a435")]
                                        // "c002_a435" => Some(tig_algorithms::vehicle_routing::c002_a435::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a436")]
                                        // "c002_a436" => Some(tig_algorithms::vehicle_routing::c002_a436::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a437")]
                                        // "c002_a437" => Some(tig_algorithms::vehicle_routing::c002_a437::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a438")]
                                        // "c002_a438" => Some(tig_algorithms::vehicle_routing::c002_a438::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a439")]
                                        // "c002_a439" => Some(tig_algorithms::vehicle_routing::c002_a439::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a440")]
                                        // "c002_a440" => Some(tig_algorithms::vehicle_routing::c002_a440::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a441")]
                                        // "c002_a441" => Some(tig_algorithms::vehicle_routing::c002_a441::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a442")]
                                        // "c002_a442" => Some(tig_algorithms::vehicle_routing::c002_a442::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a443")]
                                        // "c002_a443" => Some(tig_algorithms::vehicle_routing::c002_a443::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a444")]
                                        // "c002_a444" => Some(tig_algorithms::vehicle_routing::c002_a444::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a445")]
                                        // "c002_a445" => Some(tig_algorithms::vehicle_routing::c002_a445::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a446")]
                                        // "c002_a446" => Some(tig_algorithms::vehicle_routing::c002_a446::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a447")]
                                        // "c002_a447" => Some(tig_algorithms::vehicle_routing::c002_a447::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a448")]
                                        // "c002_a448" => Some(tig_algorithms::vehicle_routing::c002_a448::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a449")]
                                        // "c002_a449" => Some(tig_algorithms::vehicle_routing::c002_a449::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a450")]
                                        // "c002_a450" => Some(tig_algorithms::vehicle_routing::c002_a450::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a451")]
                                        // "c002_a451" => Some(tig_algorithms::vehicle_routing::c002_a451::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a452")]
                                        // "c002_a452" => Some(tig_algorithms::vehicle_routing::c002_a452::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a453")]
                                        // "c002_a453" => Some(tig_algorithms::vehicle_routing::c002_a453::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a454")]
                                        // "c002_a454" => Some(tig_algorithms::vehicle_routing::c002_a454::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a455")]
                                        // "c002_a455" => Some(tig_algorithms::vehicle_routing::c002_a455::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a456")]
                                        // "c002_a456" => Some(tig_algorithms::vehicle_routing::c002_a456::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a457")]
                                        // "c002_a457" => Some(tig_algorithms::vehicle_routing::c002_a457::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a458")]
                                        // "c002_a458" => Some(tig_algorithms::vehicle_routing::c002_a458::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a459")]
                                        // "c002_a459" => Some(tig_algorithms::vehicle_routing::c002_a459::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a460")]
                                        // "c002_a460" => Some(tig_algorithms::vehicle_routing::c002_a460::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a461")]
                                        // "c002_a461" => Some(tig_algorithms::vehicle_routing::c002_a461::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a462")]
                                        // "c002_a462" => Some(tig_algorithms::vehicle_routing::c002_a462::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a463")]
                                        // "c002_a463" => Some(tig_algorithms::vehicle_routing::c002_a463::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a464")]
                                        // "c002_a464" => Some(tig_algorithms::vehicle_routing::c002_a464::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a465")]
                                        // "c002_a465" => Some(tig_algorithms::vehicle_routing::c002_a465::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a466")]
                                        // "c002_a466" => Some(tig_algorithms::vehicle_routing::c002_a466::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a467")]
                                        // "c002_a467" => Some(tig_algorithms::vehicle_routing::c002_a467::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a468")]
                                        // "c002_a468" => Some(tig_algorithms::vehicle_routing::c002_a468::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a469")]
                                        // "c002_a469" => Some(tig_algorithms::vehicle_routing::c002_a469::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a470")]
                                        // "c002_a470" => Some(tig_algorithms::vehicle_routing::c002_a470::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a471")]
                                        // "c002_a471" => Some(tig_algorithms::vehicle_routing::c002_a471::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a472")]
                                        // "c002_a472" => Some(tig_algorithms::vehicle_routing::c002_a472::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a473")]
                                        // "c002_a473" => Some(tig_algorithms::vehicle_routing::c002_a473::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a474")]
                                        // "c002_a474" => Some(tig_algorithms::vehicle_routing::c002_a474::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a475")]
                                        // "c002_a475" => Some(tig_algorithms::vehicle_routing::c002_a475::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a476")]
                                        // "c002_a476" => Some(tig_algorithms::vehicle_routing::c002_a476::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a477")]
                                        // "c002_a477" => Some(tig_algorithms::vehicle_routing::c002_a477::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a478")]
                                        // "c002_a478" => Some(tig_algorithms::vehicle_routing::c002_a478::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a479")]
                                        // "c002_a479" => Some(tig_algorithms::vehicle_routing::c002_a479::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a480")]
                                        // "c002_a480" => Some(tig_algorithms::vehicle_routing::c002_a480::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a481")]
                                        // "c002_a481" => Some(tig_algorithms::vehicle_routing::c002_a481::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a482")]
                                        // "c002_a482" => Some(tig_algorithms::vehicle_routing::c002_a482::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a483")]
                                        // "c002_a483" => Some(tig_algorithms::vehicle_routing::c002_a483::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a484")]
                                        // "c002_a484" => Some(tig_algorithms::vehicle_routing::c002_a484::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a485")]
                                        // "c002_a485" => Some(tig_algorithms::vehicle_routing::c002_a485::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a486")]
                                        // "c002_a486" => Some(tig_algorithms::vehicle_routing::c002_a486::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a487")]
                                        // "c002_a487" => Some(tig_algorithms::vehicle_routing::c002_a487::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a488")]
                                        // "c002_a488" => Some(tig_algorithms::vehicle_routing::c002_a488::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a489")]
                                        // "c002_a489" => Some(tig_algorithms::vehicle_routing::c002_a489::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a490")]
                                        // "c002_a490" => Some(tig_algorithms::vehicle_routing::c002_a490::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a491")]
                                        // "c002_a491" => Some(tig_algorithms::vehicle_routing::c002_a491::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a492")]
                                        // "c002_a492" => Some(tig_algorithms::vehicle_routing::c002_a492::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a493")]
                                        // "c002_a493" => Some(tig_algorithms::vehicle_routing::c002_a493::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a494")]
                                        // "c002_a494" => Some(tig_algorithms::vehicle_routing::c002_a494::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a495")]
                                        // "c002_a495" => Some(tig_algorithms::vehicle_routing::c002_a495::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a496")]
                                        // "c002_a496" => Some(tig_algorithms::vehicle_routing::c002_a496::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a497")]
                                        // "c002_a497" => Some(tig_algorithms::vehicle_routing::c002_a497::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a498")]
                                        // "c002_a498" => Some(tig_algorithms::vehicle_routing::c002_a498::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a499")]
                                        // "c002_a499" => Some(tig_algorithms::vehicle_routing::c002_a499::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a500")]
                                        // "c002_a500" => Some(tig_algorithms::vehicle_routing::c002_a500::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a501")]
                                        // "c002_a501" => Some(tig_algorithms::vehicle_routing::c002_a501::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a502")]
                                        // "c002_a502" => Some(tig_algorithms::vehicle_routing::c002_a502::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a503")]
                                        // "c002_a503" => Some(tig_algorithms::vehicle_routing::c002_a503::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a504")]
                                        // "c002_a504" => Some(tig_algorithms::vehicle_routing::c002_a504::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a505")]
                                        // "c002_a505" => Some(tig_algorithms::vehicle_routing::c002_a505::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a506")]
                                        // "c002_a506" => Some(tig_algorithms::vehicle_routing::c002_a506::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a507")]
                                        // "c002_a507" => Some(tig_algorithms::vehicle_routing::c002_a507::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a508")]
                                        // "c002_a508" => Some(tig_algorithms::vehicle_routing::c002_a508::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a509")]
                                        // "c002_a509" => Some(tig_algorithms::vehicle_routing::c002_a509::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a510")]
                                        // "c002_a510" => Some(tig_algorithms::vehicle_routing::c002_a510::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a511")]
                                        // "c002_a511" => Some(tig_algorithms::vehicle_routing::c002_a511::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a512")]
                                        // "c002_a512" => Some(tig_algorithms::vehicle_routing::c002_a512::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a513")]
                                        // "c002_a513" => Some(tig_algorithms::vehicle_routing::c002_a513::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a514")]
                                        // "c002_a514" => Some(tig_algorithms::vehicle_routing::c002_a514::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a515")]
                                        // "c002_a515" => Some(tig_algorithms::vehicle_routing::c002_a515::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a516")]
                                        // "c002_a516" => Some(tig_algorithms::vehicle_routing::c002_a516::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a517")]
                                        // "c002_a517" => Some(tig_algorithms::vehicle_routing::c002_a517::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a518")]
                                        // "c002_a518" => Some(tig_algorithms::vehicle_routing::c002_a518::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a519")]
                                        // "c002_a519" => Some(tig_algorithms::vehicle_routing::c002_a519::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a520")]
                                        // "c002_a520" => Some(tig_algorithms::vehicle_routing::c002_a520::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a521")]
                                        // "c002_a521" => Some(tig_algorithms::vehicle_routing::c002_a521::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a522")]
                                        // "c002_a522" => Some(tig_algorithms::vehicle_routing::c002_a522::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a523")]
                                        // "c002_a523" => Some(tig_algorithms::vehicle_routing::c002_a523::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a524")]
                                        // "c002_a524" => Some(tig_algorithms::vehicle_routing::c002_a524::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a525")]
                                        // "c002_a525" => Some(tig_algorithms::vehicle_routing::c002_a525::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a526")]
                                        // "c002_a526" => Some(tig_algorithms::vehicle_routing::c002_a526::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a527")]
                                        // "c002_a527" => Some(tig_algorithms::vehicle_routing::c002_a527::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a528")]
                                        // "c002_a528" => Some(tig_algorithms::vehicle_routing::c002_a528::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a529")]
                                        // "c002_a529" => Some(tig_algorithms::vehicle_routing::c002_a529::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a530")]
                                        // "c002_a530" => Some(tig_algorithms::vehicle_routing::c002_a530::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a531")]
                                        // "c002_a531" => Some(tig_algorithms::vehicle_routing::c002_a531::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a532")]
                                        // "c002_a532" => Some(tig_algorithms::vehicle_routing::c002_a532::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a533")]
                                        // "c002_a533" => Some(tig_algorithms::vehicle_routing::c002_a533::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a534")]
                                        // "c002_a534" => Some(tig_algorithms::vehicle_routing::c002_a534::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a535")]
                                        // "c002_a535" => Some(tig_algorithms::vehicle_routing::c002_a535::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a536")]
                                        // "c002_a536" => Some(tig_algorithms::vehicle_routing::c002_a536::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a537")]
                                        // "c002_a537" => Some(tig_algorithms::vehicle_routing::c002_a537::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a538")]
                                        // "c002_a538" => Some(tig_algorithms::vehicle_routing::c002_a538::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a539")]
                                        // "c002_a539" => Some(tig_algorithms::vehicle_routing::c002_a539::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a540")]
                                        // "c002_a540" => Some(tig_algorithms::vehicle_routing::c002_a540::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a541")]
                                        // "c002_a541" => Some(tig_algorithms::vehicle_routing::c002_a541::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a542")]
                                        // "c002_a542" => Some(tig_algorithms::vehicle_routing::c002_a542::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a543")]
                                        // "c002_a543" => Some(tig_algorithms::vehicle_routing::c002_a543::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a544")]
                                        // "c002_a544" => Some(tig_algorithms::vehicle_routing::c002_a544::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a545")]
                                        // "c002_a545" => Some(tig_algorithms::vehicle_routing::c002_a545::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a546")]
                                        // "c002_a546" => Some(tig_algorithms::vehicle_routing::c002_a546::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a547")]
                                        // "c002_a547" => Some(tig_algorithms::vehicle_routing::c002_a547::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a548")]
                                        // "c002_a548" => Some(tig_algorithms::vehicle_routing::c002_a548::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a549")]
                                        // "c002_a549" => Some(tig_algorithms::vehicle_routing::c002_a549::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a550")]
                                        // "c002_a550" => Some(tig_algorithms::vehicle_routing::c002_a550::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a551")]
                                        // "c002_a551" => Some(tig_algorithms::vehicle_routing::c002_a551::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a552")]
                                        // "c002_a552" => Some(tig_algorithms::vehicle_routing::c002_a552::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a553")]
                                        // "c002_a553" => Some(tig_algorithms::vehicle_routing::c002_a553::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a554")]
                                        // "c002_a554" => Some(tig_algorithms::vehicle_routing::c002_a554::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a555")]
                                        // "c002_a555" => Some(tig_algorithms::vehicle_routing::c002_a555::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a556")]
                                        // "c002_a556" => Some(tig_algorithms::vehicle_routing::c002_a556::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a557")]
                                        // "c002_a557" => Some(tig_algorithms::vehicle_routing::c002_a557::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a558")]
                                        // "c002_a558" => Some(tig_algorithms::vehicle_routing::c002_a558::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a559")]
                                        // "c002_a559" => Some(tig_algorithms::vehicle_routing::c002_a559::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a560")]
                                        // "c002_a560" => Some(tig_algorithms::vehicle_routing::c002_a560::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a561")]
                                        // "c002_a561" => Some(tig_algorithms::vehicle_routing::c002_a561::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a562")]
                                        // "c002_a562" => Some(tig_algorithms::vehicle_routing::c002_a562::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a563")]
                                        // "c002_a563" => Some(tig_algorithms::vehicle_routing::c002_a563::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a564")]
                                        // "c002_a564" => Some(tig_algorithms::vehicle_routing::c002_a564::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a565")]
                                        // "c002_a565" => Some(tig_algorithms::vehicle_routing::c002_a565::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a566")]
                                        // "c002_a566" => Some(tig_algorithms::vehicle_routing::c002_a566::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a567")]
                                        // "c002_a567" => Some(tig_algorithms::vehicle_routing::c002_a567::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a568")]
                                        // "c002_a568" => Some(tig_algorithms::vehicle_routing::c002_a568::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a569")]
                                        // "c002_a569" => Some(tig_algorithms::vehicle_routing::c002_a569::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a570")]
                                        // "c002_a570" => Some(tig_algorithms::vehicle_routing::c002_a570::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a571")]
                                        // "c002_a571" => Some(tig_algorithms::vehicle_routing::c002_a571::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a572")]
                                        // "c002_a572" => Some(tig_algorithms::vehicle_routing::c002_a572::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a573")]
                                        // "c002_a573" => Some(tig_algorithms::vehicle_routing::c002_a573::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a574")]
                                        // "c002_a574" => Some(tig_algorithms::vehicle_routing::c002_a574::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a575")]
                                        // "c002_a575" => Some(tig_algorithms::vehicle_routing::c002_a575::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a576")]
                                        // "c002_a576" => Some(tig_algorithms::vehicle_routing::c002_a576::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a577")]
                                        // "c002_a577" => Some(tig_algorithms::vehicle_routing::c002_a577::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a578")]
                                        // "c002_a578" => Some(tig_algorithms::vehicle_routing::c002_a578::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a579")]
                                        // "c002_a579" => Some(tig_algorithms::vehicle_routing::c002_a579::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a580")]
                                        // "c002_a580" => Some(tig_algorithms::vehicle_routing::c002_a580::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a581")]
                                        // "c002_a581" => Some(tig_algorithms::vehicle_routing::c002_a581::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a582")]
                                        // "c002_a582" => Some(tig_algorithms::vehicle_routing::c002_a582::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a583")]
                                        // "c002_a583" => Some(tig_algorithms::vehicle_routing::c002_a583::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a584")]
                                        // "c002_a584" => Some(tig_algorithms::vehicle_routing::c002_a584::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a585")]
                                        // "c002_a585" => Some(tig_algorithms::vehicle_routing::c002_a585::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a586")]
                                        // "c002_a586" => Some(tig_algorithms::vehicle_routing::c002_a586::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a587")]
                                        // "c002_a587" => Some(tig_algorithms::vehicle_routing::c002_a587::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a588")]
                                        // "c002_a588" => Some(tig_algorithms::vehicle_routing::c002_a588::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a589")]
                                        // "c002_a589" => Some(tig_algorithms::vehicle_routing::c002_a589::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a590")]
                                        // "c002_a590" => Some(tig_algorithms::vehicle_routing::c002_a590::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a591")]
                                        // "c002_a591" => Some(tig_algorithms::vehicle_routing::c002_a591::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a592")]
                                        // "c002_a592" => Some(tig_algorithms::vehicle_routing::c002_a592::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a593")]
                                        // "c002_a593" => Some(tig_algorithms::vehicle_routing::c002_a593::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a594")]
                                        // "c002_a594" => Some(tig_algorithms::vehicle_routing::c002_a594::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a595")]
                                        // "c002_a595" => Some(tig_algorithms::vehicle_routing::c002_a595::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a596")]
                                        // "c002_a596" => Some(tig_algorithms::vehicle_routing::c002_a596::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a597")]
                                        // "c002_a597" => Some(tig_algorithms::vehicle_routing::c002_a597::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a598")]
                                        // "c002_a598" => Some(tig_algorithms::vehicle_routing::c002_a598::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a599")]
                                        // "c002_a599" => Some(tig_algorithms::vehicle_routing::c002_a599::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a600")]
                                        // "c002_a600" => Some(tig_algorithms::vehicle_routing::c002_a600::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a601")]
                                        // "c002_a601" => Some(tig_algorithms::vehicle_routing::c002_a601::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a602")]
                                        // "c002_a602" => Some(tig_algorithms::vehicle_routing::c002_a602::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a603")]
                                        // "c002_a603" => Some(tig_algorithms::vehicle_routing::c002_a603::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a604")]
                                        // "c002_a604" => Some(tig_algorithms::vehicle_routing::c002_a604::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a605")]
                                        // "c002_a605" => Some(tig_algorithms::vehicle_routing::c002_a605::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a606")]
                                        // "c002_a606" => Some(tig_algorithms::vehicle_routing::c002_a606::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a607")]
                                        // "c002_a607" => Some(tig_algorithms::vehicle_routing::c002_a607::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a608")]
                                        // "c002_a608" => Some(tig_algorithms::vehicle_routing::c002_a608::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a609")]
                                        // "c002_a609" => Some(tig_algorithms::vehicle_routing::c002_a609::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a610")]
                                        // "c002_a610" => Some(tig_algorithms::vehicle_routing::c002_a610::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a611")]
                                        // "c002_a611" => Some(tig_algorithms::vehicle_routing::c002_a611::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a612")]
                                        // "c002_a612" => Some(tig_algorithms::vehicle_routing::c002_a612::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a613")]
                                        // "c002_a613" => Some(tig_algorithms::vehicle_routing::c002_a613::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a614")]
                                        // "c002_a614" => Some(tig_algorithms::vehicle_routing::c002_a614::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a615")]
                                        // "c002_a615" => Some(tig_algorithms::vehicle_routing::c002_a615::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a616")]
                                        // "c002_a616" => Some(tig_algorithms::vehicle_routing::c002_a616::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a617")]
                                        // "c002_a617" => Some(tig_algorithms::vehicle_routing::c002_a617::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a618")]
                                        // "c002_a618" => Some(tig_algorithms::vehicle_routing::c002_a618::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a619")]
                                        // "c002_a619" => Some(tig_algorithms::vehicle_routing::c002_a619::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a620")]
                                        // "c002_a620" => Some(tig_algorithms::vehicle_routing::c002_a620::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a621")]
                                        // "c002_a621" => Some(tig_algorithms::vehicle_routing::c002_a621::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a622")]
                                        // "c002_a622" => Some(tig_algorithms::vehicle_routing::c002_a622::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a623")]
                                        // "c002_a623" => Some(tig_algorithms::vehicle_routing::c002_a623::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a624")]
                                        // "c002_a624" => Some(tig_algorithms::vehicle_routing::c002_a624::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a625")]
                                        // "c002_a625" => Some(tig_algorithms::vehicle_routing::c002_a625::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a626")]
                                        // "c002_a626" => Some(tig_algorithms::vehicle_routing::c002_a626::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a627")]
                                        // "c002_a627" => Some(tig_algorithms::vehicle_routing::c002_a627::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a628")]
                                        // "c002_a628" => Some(tig_algorithms::vehicle_routing::c002_a628::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a629")]
                                        // "c002_a629" => Some(tig_algorithms::vehicle_routing::c002_a629::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a630")]
                                        // "c002_a630" => Some(tig_algorithms::vehicle_routing::c002_a630::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a631")]
                                        // "c002_a631" => Some(tig_algorithms::vehicle_routing::c002_a631::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a632")]
                                        // "c002_a632" => Some(tig_algorithms::vehicle_routing::c002_a632::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a633")]
                                        // "c002_a633" => Some(tig_algorithms::vehicle_routing::c002_a633::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a634")]
                                        // "c002_a634" => Some(tig_algorithms::vehicle_routing::c002_a634::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a635")]
                                        // "c002_a635" => Some(tig_algorithms::vehicle_routing::c002_a635::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a636")]
                                        // "c002_a636" => Some(tig_algorithms::vehicle_routing::c002_a636::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a637")]
                                        // "c002_a637" => Some(tig_algorithms::vehicle_routing::c002_a637::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a638")]
                                        // "c002_a638" => Some(tig_algorithms::vehicle_routing::c002_a638::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a639")]
                                        // "c002_a639" => Some(tig_algorithms::vehicle_routing::c002_a639::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a640")]
                                        // "c002_a640" => Some(tig_algorithms::vehicle_routing::c002_a640::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a641")]
                                        // "c002_a641" => Some(tig_algorithms::vehicle_routing::c002_a641::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a642")]
                                        // "c002_a642" => Some(tig_algorithms::vehicle_routing::c002_a642::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a643")]
                                        // "c002_a643" => Some(tig_algorithms::vehicle_routing::c002_a643::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a644")]
                                        // "c002_a644" => Some(tig_algorithms::vehicle_routing::c002_a644::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a645")]
                                        // "c002_a645" => Some(tig_algorithms::vehicle_routing::c002_a645::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a646")]
                                        // "c002_a646" => Some(tig_algorithms::vehicle_routing::c002_a646::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a647")]
                                        // "c002_a647" => Some(tig_algorithms::vehicle_routing::c002_a647::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a648")]
                                        // "c002_a648" => Some(tig_algorithms::vehicle_routing::c002_a648::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a649")]
                                        // "c002_a649" => Some(tig_algorithms::vehicle_routing::c002_a649::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a650")]
                                        // "c002_a650" => Some(tig_algorithms::vehicle_routing::c002_a650::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a651")]
                                        // "c002_a651" => Some(tig_algorithms::vehicle_routing::c002_a651::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a652")]
                                        // "c002_a652" => Some(tig_algorithms::vehicle_routing::c002_a652::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a653")]
                                        // "c002_a653" => Some(tig_algorithms::vehicle_routing::c002_a653::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a654")]
                                        // "c002_a654" => Some(tig_algorithms::vehicle_routing::c002_a654::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a655")]
                                        // "c002_a655" => Some(tig_algorithms::vehicle_routing::c002_a655::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a656")]
                                        // "c002_a656" => Some(tig_algorithms::vehicle_routing::c002_a656::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a657")]
                                        // "c002_a657" => Some(tig_algorithms::vehicle_routing::c002_a657::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a658")]
                                        // "c002_a658" => Some(tig_algorithms::vehicle_routing::c002_a658::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a659")]
                                        // "c002_a659" => Some(tig_algorithms::vehicle_routing::c002_a659::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a660")]
                                        // "c002_a660" => Some(tig_algorithms::vehicle_routing::c002_a660::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a661")]
                                        // "c002_a661" => Some(tig_algorithms::vehicle_routing::c002_a661::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a662")]
                                        // "c002_a662" => Some(tig_algorithms::vehicle_routing::c002_a662::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a663")]
                                        // "c002_a663" => Some(tig_algorithms::vehicle_routing::c002_a663::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a664")]
                                        // "c002_a664" => Some(tig_algorithms::vehicle_routing::c002_a664::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a665")]
                                        // "c002_a665" => Some(tig_algorithms::vehicle_routing::c002_a665::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a666")]
                                        // "c002_a666" => Some(tig_algorithms::vehicle_routing::c002_a666::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a667")]
                                        // "c002_a667" => Some(tig_algorithms::vehicle_routing::c002_a667::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a668")]
                                        // "c002_a668" => Some(tig_algorithms::vehicle_routing::c002_a668::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a669")]
                                        // "c002_a669" => Some(tig_algorithms::vehicle_routing::c002_a669::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a670")]
                                        // "c002_a670" => Some(tig_algorithms::vehicle_routing::c002_a670::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a671")]
                                        // "c002_a671" => Some(tig_algorithms::vehicle_routing::c002_a671::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a672")]
                                        // "c002_a672" => Some(tig_algorithms::vehicle_routing::c002_a672::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a673")]
                                        // "c002_a673" => Some(tig_algorithms::vehicle_routing::c002_a673::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a674")]
                                        // "c002_a674" => Some(tig_algorithms::vehicle_routing::c002_a674::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a675")]
                                        // "c002_a675" => Some(tig_algorithms::vehicle_routing::c002_a675::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a676")]
                                        // "c002_a676" => Some(tig_algorithms::vehicle_routing::c002_a676::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a677")]
                                        // "c002_a677" => Some(tig_algorithms::vehicle_routing::c002_a677::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a678")]
                                        // "c002_a678" => Some(tig_algorithms::vehicle_routing::c002_a678::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a679")]
                                        // "c002_a679" => Some(tig_algorithms::vehicle_routing::c002_a679::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a680")]
                                        // "c002_a680" => Some(tig_algorithms::vehicle_routing::c002_a680::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a681")]
                                        // "c002_a681" => Some(tig_algorithms::vehicle_routing::c002_a681::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a682")]
                                        // "c002_a682" => Some(tig_algorithms::vehicle_routing::c002_a682::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a683")]
                                        // "c002_a683" => Some(tig_algorithms::vehicle_routing::c002_a683::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a684")]
                                        // "c002_a684" => Some(tig_algorithms::vehicle_routing::c002_a684::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a685")]
                                        // "c002_a685" => Some(tig_algorithms::vehicle_routing::c002_a685::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a686")]
                                        // "c002_a686" => Some(tig_algorithms::vehicle_routing::c002_a686::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a687")]
                                        // "c002_a687" => Some(tig_algorithms::vehicle_routing::c002_a687::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a688")]
                                        // "c002_a688" => Some(tig_algorithms::vehicle_routing::c002_a688::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a689")]
                                        // "c002_a689" => Some(tig_algorithms::vehicle_routing::c002_a689::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a690")]
                                        // "c002_a690" => Some(tig_algorithms::vehicle_routing::c002_a690::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a691")]
                                        // "c002_a691" => Some(tig_algorithms::vehicle_routing::c002_a691::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a692")]
                                        // "c002_a692" => Some(tig_algorithms::vehicle_routing::c002_a692::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a693")]
                                        // "c002_a693" => Some(tig_algorithms::vehicle_routing::c002_a693::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a694")]
                                        // "c002_a694" => Some(tig_algorithms::vehicle_routing::c002_a694::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a695")]
                                        // "c002_a695" => Some(tig_algorithms::vehicle_routing::c002_a695::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a696")]
                                        // "c002_a696" => Some(tig_algorithms::vehicle_routing::c002_a696::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a697")]
                                        // "c002_a697" => Some(tig_algorithms::vehicle_routing::c002_a697::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a698")]
                                        // "c002_a698" => Some(tig_algorithms::vehicle_routing::c002_a698::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a699")]
                                        // "c002_a699" => Some(tig_algorithms::vehicle_routing::c002_a699::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a700")]
                                        // "c002_a700" => Some(tig_algorithms::vehicle_routing::c002_a700::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a701")]
                                        // "c002_a701" => Some(tig_algorithms::vehicle_routing::c002_a701::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a702")]
                                        // "c002_a702" => Some(tig_algorithms::vehicle_routing::c002_a702::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a703")]
                                        // "c002_a703" => Some(tig_algorithms::vehicle_routing::c002_a703::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a704")]
                                        // "c002_a704" => Some(tig_algorithms::vehicle_routing::c002_a704::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a705")]
                                        // "c002_a705" => Some(tig_algorithms::vehicle_routing::c002_a705::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a706")]
                                        // "c002_a706" => Some(tig_algorithms::vehicle_routing::c002_a706::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a707")]
                                        // "c002_a707" => Some(tig_algorithms::vehicle_routing::c002_a707::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a708")]
                                        // "c002_a708" => Some(tig_algorithms::vehicle_routing::c002_a708::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a709")]
                                        // "c002_a709" => Some(tig_algorithms::vehicle_routing::c002_a709::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a710")]
                                        // "c002_a710" => Some(tig_algorithms::vehicle_routing::c002_a710::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a711")]
                                        // "c002_a711" => Some(tig_algorithms::vehicle_routing::c002_a711::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a712")]
                                        // "c002_a712" => Some(tig_algorithms::vehicle_routing::c002_a712::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a713")]
                                        // "c002_a713" => Some(tig_algorithms::vehicle_routing::c002_a713::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a714")]
                                        // "c002_a714" => Some(tig_algorithms::vehicle_routing::c002_a714::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a715")]
                                        // "c002_a715" => Some(tig_algorithms::vehicle_routing::c002_a715::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a716")]
                                        // "c002_a716" => Some(tig_algorithms::vehicle_routing::c002_a716::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a717")]
                                        // "c002_a717" => Some(tig_algorithms::vehicle_routing::c002_a717::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a718")]
                                        // "c002_a718" => Some(tig_algorithms::vehicle_routing::c002_a718::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a719")]
                                        // "c002_a719" => Some(tig_algorithms::vehicle_routing::c002_a719::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a720")]
                                        // "c002_a720" => Some(tig_algorithms::vehicle_routing::c002_a720::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a721")]
                                        // "c002_a721" => Some(tig_algorithms::vehicle_routing::c002_a721::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a722")]
                                        // "c002_a722" => Some(tig_algorithms::vehicle_routing::c002_a722::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a723")]
                                        // "c002_a723" => Some(tig_algorithms::vehicle_routing::c002_a723::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a724")]
                                        // "c002_a724" => Some(tig_algorithms::vehicle_routing::c002_a724::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a725")]
                                        // "c002_a725" => Some(tig_algorithms::vehicle_routing::c002_a725::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a726")]
                                        // "c002_a726" => Some(tig_algorithms::vehicle_routing::c002_a726::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a727")]
                                        // "c002_a727" => Some(tig_algorithms::vehicle_routing::c002_a727::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a728")]
                                        // "c002_a728" => Some(tig_algorithms::vehicle_routing::c002_a728::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a729")]
                                        // "c002_a729" => Some(tig_algorithms::vehicle_routing::c002_a729::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a730")]
                                        // "c002_a730" => Some(tig_algorithms::vehicle_routing::c002_a730::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a731")]
                                        // "c002_a731" => Some(tig_algorithms::vehicle_routing::c002_a731::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a732")]
                                        // "c002_a732" => Some(tig_algorithms::vehicle_routing::c002_a732::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a733")]
                                        // "c002_a733" => Some(tig_algorithms::vehicle_routing::c002_a733::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a734")]
                                        // "c002_a734" => Some(tig_algorithms::vehicle_routing::c002_a734::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a735")]
                                        // "c002_a735" => Some(tig_algorithms::vehicle_routing::c002_a735::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a736")]
                                        // "c002_a736" => Some(tig_algorithms::vehicle_routing::c002_a736::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a737")]
                                        // "c002_a737" => Some(tig_algorithms::vehicle_routing::c002_a737::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a738")]
                                        // "c002_a738" => Some(tig_algorithms::vehicle_routing::c002_a738::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a739")]
                                        // "c002_a739" => Some(tig_algorithms::vehicle_routing::c002_a739::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a740")]
                                        // "c002_a740" => Some(tig_algorithms::vehicle_routing::c002_a740::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a741")]
                                        // "c002_a741" => Some(tig_algorithms::vehicle_routing::c002_a741::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a742")]
                                        // "c002_a742" => Some(tig_algorithms::vehicle_routing::c002_a742::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a743")]
                                        // "c002_a743" => Some(tig_algorithms::vehicle_routing::c002_a743::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a744")]
                                        // "c002_a744" => Some(tig_algorithms::vehicle_routing::c002_a744::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a745")]
                                        // "c002_a745" => Some(tig_algorithms::vehicle_routing::c002_a745::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a746")]
                                        // "c002_a746" => Some(tig_algorithms::vehicle_routing::c002_a746::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a747")]
                                        // "c002_a747" => Some(tig_algorithms::vehicle_routing::c002_a747::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a748")]
                                        // "c002_a748" => Some(tig_algorithms::vehicle_routing::c002_a748::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a749")]
                                        // "c002_a749" => Some(tig_algorithms::vehicle_routing::c002_a749::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a750")]
                                        // "c002_a750" => Some(tig_algorithms::vehicle_routing::c002_a750::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a751")]
                                        // "c002_a751" => Some(tig_algorithms::vehicle_routing::c002_a751::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a752")]
                                        // "c002_a752" => Some(tig_algorithms::vehicle_routing::c002_a752::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a753")]
                                        // "c002_a753" => Some(tig_algorithms::vehicle_routing::c002_a753::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a754")]
                                        // "c002_a754" => Some(tig_algorithms::vehicle_routing::c002_a754::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a755")]
                                        // "c002_a755" => Some(tig_algorithms::vehicle_routing::c002_a755::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a756")]
                                        // "c002_a756" => Some(tig_algorithms::vehicle_routing::c002_a756::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a757")]
                                        // "c002_a757" => Some(tig_algorithms::vehicle_routing::c002_a757::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a758")]
                                        // "c002_a758" => Some(tig_algorithms::vehicle_routing::c002_a758::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a759")]
                                        // "c002_a759" => Some(tig_algorithms::vehicle_routing::c002_a759::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a760")]
                                        // "c002_a760" => Some(tig_algorithms::vehicle_routing::c002_a760::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a761")]
                                        // "c002_a761" => Some(tig_algorithms::vehicle_routing::c002_a761::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a762")]
                                        // "c002_a762" => Some(tig_algorithms::vehicle_routing::c002_a762::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a763")]
                                        // "c002_a763" => Some(tig_algorithms::vehicle_routing::c002_a763::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a764")]
                                        // "c002_a764" => Some(tig_algorithms::vehicle_routing::c002_a764::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a765")]
                                        // "c002_a765" => Some(tig_algorithms::vehicle_routing::c002_a765::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a766")]
                                        // "c002_a766" => Some(tig_algorithms::vehicle_routing::c002_a766::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a767")]
                                        // "c002_a767" => Some(tig_algorithms::vehicle_routing::c002_a767::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a768")]
                                        // "c002_a768" => Some(tig_algorithms::vehicle_routing::c002_a768::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a769")]
                                        // "c002_a769" => Some(tig_algorithms::vehicle_routing::c002_a769::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a770")]
                                        // "c002_a770" => Some(tig_algorithms::vehicle_routing::c002_a770::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a771")]
                                        // "c002_a771" => Some(tig_algorithms::vehicle_routing::c002_a771::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a772")]
                                        // "c002_a772" => Some(tig_algorithms::vehicle_routing::c002_a772::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a773")]
                                        // "c002_a773" => Some(tig_algorithms::vehicle_routing::c002_a773::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a774")]
                                        // "c002_a774" => Some(tig_algorithms::vehicle_routing::c002_a774::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a775")]
                                        // "c002_a775" => Some(tig_algorithms::vehicle_routing::c002_a775::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a776")]
                                        // "c002_a776" => Some(tig_algorithms::vehicle_routing::c002_a776::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a777")]
                                        // "c002_a777" => Some(tig_algorithms::vehicle_routing::c002_a777::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a778")]
                                        // "c002_a778" => Some(tig_algorithms::vehicle_routing::c002_a778::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a779")]
                                        // "c002_a779" => Some(tig_algorithms::vehicle_routing::c002_a779::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a780")]
                                        // "c002_a780" => Some(tig_algorithms::vehicle_routing::c002_a780::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a781")]
                                        // "c002_a781" => Some(tig_algorithms::vehicle_routing::c002_a781::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a782")]
                                        // "c002_a782" => Some(tig_algorithms::vehicle_routing::c002_a782::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a783")]
                                        // "c002_a783" => Some(tig_algorithms::vehicle_routing::c002_a783::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a784")]
                                        // "c002_a784" => Some(tig_algorithms::vehicle_routing::c002_a784::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a785")]
                                        // "c002_a785" => Some(tig_algorithms::vehicle_routing::c002_a785::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a786")]
                                        // "c002_a786" => Some(tig_algorithms::vehicle_routing::c002_a786::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a787")]
                                        // "c002_a787" => Some(tig_algorithms::vehicle_routing::c002_a787::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a788")]
                                        // "c002_a788" => Some(tig_algorithms::vehicle_routing::c002_a788::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a789")]
                                        // "c002_a789" => Some(tig_algorithms::vehicle_routing::c002_a789::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a790")]
                                        // "c002_a790" => Some(tig_algorithms::vehicle_routing::c002_a790::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a791")]
                                        // "c002_a791" => Some(tig_algorithms::vehicle_routing::c002_a791::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a792")]
                                        // "c002_a792" => Some(tig_algorithms::vehicle_routing::c002_a792::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a793")]
                                        // "c002_a793" => Some(tig_algorithms::vehicle_routing::c002_a793::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a794")]
                                        // "c002_a794" => Some(tig_algorithms::vehicle_routing::c002_a794::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a795")]
                                        // "c002_a795" => Some(tig_algorithms::vehicle_routing::c002_a795::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a796")]
                                        // "c002_a796" => Some(tig_algorithms::vehicle_routing::c002_a796::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a797")]
                                        // "c002_a797" => Some(tig_algorithms::vehicle_routing::c002_a797::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a798")]
                                        // "c002_a798" => Some(tig_algorithms::vehicle_routing::c002_a798::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a799")]
                                        // "c002_a799" => Some(tig_algorithms::vehicle_routing::c002_a799::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a800")]
                                        // "c002_a800" => Some(tig_algorithms::vehicle_routing::c002_a800::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a801")]
                                        // "c002_a801" => Some(tig_algorithms::vehicle_routing::c002_a801::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a802")]
                                        // "c002_a802" => Some(tig_algorithms::vehicle_routing::c002_a802::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a803")]
                                        // "c002_a803" => Some(tig_algorithms::vehicle_routing::c002_a803::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a804")]
                                        // "c002_a804" => Some(tig_algorithms::vehicle_routing::c002_a804::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a805")]
                                        // "c002_a805" => Some(tig_algorithms::vehicle_routing::c002_a805::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a806")]
                                        // "c002_a806" => Some(tig_algorithms::vehicle_routing::c002_a806::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a807")]
                                        // "c002_a807" => Some(tig_algorithms::vehicle_routing::c002_a807::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a808")]
                                        // "c002_a808" => Some(tig_algorithms::vehicle_routing::c002_a808::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a809")]
                                        // "c002_a809" => Some(tig_algorithms::vehicle_routing::c002_a809::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a810")]
                                        // "c002_a810" => Some(tig_algorithms::vehicle_routing::c002_a810::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a811")]
                                        // "c002_a811" => Some(tig_algorithms::vehicle_routing::c002_a811::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a812")]
                                        // "c002_a812" => Some(tig_algorithms::vehicle_routing::c002_a812::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a813")]
                                        // "c002_a813" => Some(tig_algorithms::vehicle_routing::c002_a813::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a814")]
                                        // "c002_a814" => Some(tig_algorithms::vehicle_routing::c002_a814::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a815")]
                                        // "c002_a815" => Some(tig_algorithms::vehicle_routing::c002_a815::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a816")]
                                        // "c002_a816" => Some(tig_algorithms::vehicle_routing::c002_a816::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a817")]
                                        // "c002_a817" => Some(tig_algorithms::vehicle_routing::c002_a817::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a818")]
                                        // "c002_a818" => Some(tig_algorithms::vehicle_routing::c002_a818::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a819")]
                                        // "c002_a819" => Some(tig_algorithms::vehicle_routing::c002_a819::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a820")]
                                        // "c002_a820" => Some(tig_algorithms::vehicle_routing::c002_a820::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a821")]
                                        // "c002_a821" => Some(tig_algorithms::vehicle_routing::c002_a821::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a822")]
                                        // "c002_a822" => Some(tig_algorithms::vehicle_routing::c002_a822::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a823")]
                                        // "c002_a823" => Some(tig_algorithms::vehicle_routing::c002_a823::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a824")]
                                        // "c002_a824" => Some(tig_algorithms::vehicle_routing::c002_a824::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a825")]
                                        // "c002_a825" => Some(tig_algorithms::vehicle_routing::c002_a825::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a826")]
                                        // "c002_a826" => Some(tig_algorithms::vehicle_routing::c002_a826::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a827")]
                                        // "c002_a827" => Some(tig_algorithms::vehicle_routing::c002_a827::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a828")]
                                        // "c002_a828" => Some(tig_algorithms::vehicle_routing::c002_a828::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a829")]
                                        // "c002_a829" => Some(tig_algorithms::vehicle_routing::c002_a829::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a830")]
                                        // "c002_a830" => Some(tig_algorithms::vehicle_routing::c002_a830::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a831")]
                                        // "c002_a831" => Some(tig_algorithms::vehicle_routing::c002_a831::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a832")]
                                        // "c002_a832" => Some(tig_algorithms::vehicle_routing::c002_a832::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a833")]
                                        // "c002_a833" => Some(tig_algorithms::vehicle_routing::c002_a833::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a834")]
                                        // "c002_a834" => Some(tig_algorithms::vehicle_routing::c002_a834::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a835")]
                                        // "c002_a835" => Some(tig_algorithms::vehicle_routing::c002_a835::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a836")]
                                        // "c002_a836" => Some(tig_algorithms::vehicle_routing::c002_a836::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a837")]
                                        // "c002_a837" => Some(tig_algorithms::vehicle_routing::c002_a837::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a838")]
                                        // "c002_a838" => Some(tig_algorithms::vehicle_routing::c002_a838::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a839")]
                                        // "c002_a839" => Some(tig_algorithms::vehicle_routing::c002_a839::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a840")]
                                        // "c002_a840" => Some(tig_algorithms::vehicle_routing::c002_a840::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a841")]
                                        // "c002_a841" => Some(tig_algorithms::vehicle_routing::c002_a841::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a842")]
                                        // "c002_a842" => Some(tig_algorithms::vehicle_routing::c002_a842::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a843")]
                                        // "c002_a843" => Some(tig_algorithms::vehicle_routing::c002_a843::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a844")]
                                        // "c002_a844" => Some(tig_algorithms::vehicle_routing::c002_a844::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a845")]
                                        // "c002_a845" => Some(tig_algorithms::vehicle_routing::c002_a845::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a846")]
                                        // "c002_a846" => Some(tig_algorithms::vehicle_routing::c002_a846::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a847")]
                                        // "c002_a847" => Some(tig_algorithms::vehicle_routing::c002_a847::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a848")]
                                        // "c002_a848" => Some(tig_algorithms::vehicle_routing::c002_a848::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a849")]
                                        // "c002_a849" => Some(tig_algorithms::vehicle_routing::c002_a849::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a850")]
                                        // "c002_a850" => Some(tig_algorithms::vehicle_routing::c002_a850::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a851")]
                                        // "c002_a851" => Some(tig_algorithms::vehicle_routing::c002_a851::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a852")]
                                        // "c002_a852" => Some(tig_algorithms::vehicle_routing::c002_a852::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a853")]
                                        // "c002_a853" => Some(tig_algorithms::vehicle_routing::c002_a853::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a854")]
                                        // "c002_a854" => Some(tig_algorithms::vehicle_routing::c002_a854::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a855")]
                                        // "c002_a855" => Some(tig_algorithms::vehicle_routing::c002_a855::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a856")]
                                        // "c002_a856" => Some(tig_algorithms::vehicle_routing::c002_a856::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a857")]
                                        // "c002_a857" => Some(tig_algorithms::vehicle_routing::c002_a857::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a858")]
                                        // "c002_a858" => Some(tig_algorithms::vehicle_routing::c002_a858::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a859")]
                                        // "c002_a859" => Some(tig_algorithms::vehicle_routing::c002_a859::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a860")]
                                        // "c002_a860" => Some(tig_algorithms::vehicle_routing::c002_a860::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a861")]
                                        // "c002_a861" => Some(tig_algorithms::vehicle_routing::c002_a861::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a862")]
                                        // "c002_a862" => Some(tig_algorithms::vehicle_routing::c002_a862::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a863")]
                                        // "c002_a863" => Some(tig_algorithms::vehicle_routing::c002_a863::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a864")]
                                        // "c002_a864" => Some(tig_algorithms::vehicle_routing::c002_a864::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a865")]
                                        // "c002_a865" => Some(tig_algorithms::vehicle_routing::c002_a865::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a866")]
                                        // "c002_a866" => Some(tig_algorithms::vehicle_routing::c002_a866::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a867")]
                                        // "c002_a867" => Some(tig_algorithms::vehicle_routing::c002_a867::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a868")]
                                        // "c002_a868" => Some(tig_algorithms::vehicle_routing::c002_a868::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a869")]
                                        // "c002_a869" => Some(tig_algorithms::vehicle_routing::c002_a869::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a870")]
                                        // "c002_a870" => Some(tig_algorithms::vehicle_routing::c002_a870::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a871")]
                                        // "c002_a871" => Some(tig_algorithms::vehicle_routing::c002_a871::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a872")]
                                        // "c002_a872" => Some(tig_algorithms::vehicle_routing::c002_a872::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a873")]
                                        // "c002_a873" => Some(tig_algorithms::vehicle_routing::c002_a873::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a874")]
                                        // "c002_a874" => Some(tig_algorithms::vehicle_routing::c002_a874::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a875")]
                                        // "c002_a875" => Some(tig_algorithms::vehicle_routing::c002_a875::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a876")]
                                        // "c002_a876" => Some(tig_algorithms::vehicle_routing::c002_a876::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a877")]
                                        // "c002_a877" => Some(tig_algorithms::vehicle_routing::c002_a877::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a878")]
                                        // "c002_a878" => Some(tig_algorithms::vehicle_routing::c002_a878::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a879")]
                                        // "c002_a879" => Some(tig_algorithms::vehicle_routing::c002_a879::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a880")]
                                        // "c002_a880" => Some(tig_algorithms::vehicle_routing::c002_a880::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a881")]
                                        // "c002_a881" => Some(tig_algorithms::vehicle_routing::c002_a881::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a882")]
                                        // "c002_a882" => Some(tig_algorithms::vehicle_routing::c002_a882::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a883")]
                                        // "c002_a883" => Some(tig_algorithms::vehicle_routing::c002_a883::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a884")]
                                        // "c002_a884" => Some(tig_algorithms::vehicle_routing::c002_a884::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a885")]
                                        // "c002_a885" => Some(tig_algorithms::vehicle_routing::c002_a885::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a886")]
                                        // "c002_a886" => Some(tig_algorithms::vehicle_routing::c002_a886::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a887")]
                                        // "c002_a887" => Some(tig_algorithms::vehicle_routing::c002_a887::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a888")]
                                        // "c002_a888" => Some(tig_algorithms::vehicle_routing::c002_a888::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a889")]
                                        // "c002_a889" => Some(tig_algorithms::vehicle_routing::c002_a889::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a890")]
                                        // "c002_a890" => Some(tig_algorithms::vehicle_routing::c002_a890::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a891")]
                                        // "c002_a891" => Some(tig_algorithms::vehicle_routing::c002_a891::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a892")]
                                        // "c002_a892" => Some(tig_algorithms::vehicle_routing::c002_a892::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a893")]
                                        // "c002_a893" => Some(tig_algorithms::vehicle_routing::c002_a893::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a894")]
                                        // "c002_a894" => Some(tig_algorithms::vehicle_routing::c002_a894::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a895")]
                                        // "c002_a895" => Some(tig_algorithms::vehicle_routing::c002_a895::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a896")]
                                        // "c002_a896" => Some(tig_algorithms::vehicle_routing::c002_a896::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a897")]
                                        // "c002_a897" => Some(tig_algorithms::vehicle_routing::c002_a897::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a898")]
                                        // "c002_a898" => Some(tig_algorithms::vehicle_routing::c002_a898::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a899")]
                                        // "c002_a899" => Some(tig_algorithms::vehicle_routing::c002_a899::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a900")]
                                        // "c002_a900" => Some(tig_algorithms::vehicle_routing::c002_a900::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a901")]
                                        // "c002_a901" => Some(tig_algorithms::vehicle_routing::c002_a901::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a902")]
                                        // "c002_a902" => Some(tig_algorithms::vehicle_routing::c002_a902::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a903")]
                                        // "c002_a903" => Some(tig_algorithms::vehicle_routing::c002_a903::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a904")]
                                        // "c002_a904" => Some(tig_algorithms::vehicle_routing::c002_a904::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a905")]
                                        // "c002_a905" => Some(tig_algorithms::vehicle_routing::c002_a905::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a906")]
                                        // "c002_a906" => Some(tig_algorithms::vehicle_routing::c002_a906::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a907")]
                                        // "c002_a907" => Some(tig_algorithms::vehicle_routing::c002_a907::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a908")]
                                        // "c002_a908" => Some(tig_algorithms::vehicle_routing::c002_a908::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a909")]
                                        // "c002_a909" => Some(tig_algorithms::vehicle_routing::c002_a909::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a910")]
                                        // "c002_a910" => Some(tig_algorithms::vehicle_routing::c002_a910::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a911")]
                                        // "c002_a911" => Some(tig_algorithms::vehicle_routing::c002_a911::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a912")]
                                        // "c002_a912" => Some(tig_algorithms::vehicle_routing::c002_a912::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a913")]
                                        // "c002_a913" => Some(tig_algorithms::vehicle_routing::c002_a913::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a914")]
                                        // "c002_a914" => Some(tig_algorithms::vehicle_routing::c002_a914::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a915")]
                                        // "c002_a915" => Some(tig_algorithms::vehicle_routing::c002_a915::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a916")]
                                        // "c002_a916" => Some(tig_algorithms::vehicle_routing::c002_a916::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a917")]
                                        // "c002_a917" => Some(tig_algorithms::vehicle_routing::c002_a917::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a918")]
                                        // "c002_a918" => Some(tig_algorithms::vehicle_routing::c002_a918::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a919")]
                                        // "c002_a919" => Some(tig_algorithms::vehicle_routing::c002_a919::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a920")]
                                        // "c002_a920" => Some(tig_algorithms::vehicle_routing::c002_a920::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a921")]
                                        // "c002_a921" => Some(tig_algorithms::vehicle_routing::c002_a921::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a922")]
                                        // "c002_a922" => Some(tig_algorithms::vehicle_routing::c002_a922::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a923")]
                                        // "c002_a923" => Some(tig_algorithms::vehicle_routing::c002_a923::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a924")]
                                        // "c002_a924" => Some(tig_algorithms::vehicle_routing::c002_a924::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a925")]
                                        // "c002_a925" => Some(tig_algorithms::vehicle_routing::c002_a925::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a926")]
                                        // "c002_a926" => Some(tig_algorithms::vehicle_routing::c002_a926::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a927")]
                                        // "c002_a927" => Some(tig_algorithms::vehicle_routing::c002_a927::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a928")]
                                        // "c002_a928" => Some(tig_algorithms::vehicle_routing::c002_a928::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a929")]
                                        // "c002_a929" => Some(tig_algorithms::vehicle_routing::c002_a929::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a930")]
                                        // "c002_a930" => Some(tig_algorithms::vehicle_routing::c002_a930::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a931")]
                                        // "c002_a931" => Some(tig_algorithms::vehicle_routing::c002_a931::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a932")]
                                        // "c002_a932" => Some(tig_algorithms::vehicle_routing::c002_a932::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a933")]
                                        // "c002_a933" => Some(tig_algorithms::vehicle_routing::c002_a933::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a934")]
                                        // "c002_a934" => Some(tig_algorithms::vehicle_routing::c002_a934::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a935")]
                                        // "c002_a935" => Some(tig_algorithms::vehicle_routing::c002_a935::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a936")]
                                        // "c002_a936" => Some(tig_algorithms::vehicle_routing::c002_a936::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a937")]
                                        // "c002_a937" => Some(tig_algorithms::vehicle_routing::c002_a937::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a938")]
                                        // "c002_a938" => Some(tig_algorithms::vehicle_routing::c002_a938::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a939")]
                                        // "c002_a939" => Some(tig_algorithms::vehicle_routing::c002_a939::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a940")]
                                        // "c002_a940" => Some(tig_algorithms::vehicle_routing::c002_a940::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a941")]
                                        // "c002_a941" => Some(tig_algorithms::vehicle_routing::c002_a941::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a942")]
                                        // "c002_a942" => Some(tig_algorithms::vehicle_routing::c002_a942::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a943")]
                                        // "c002_a943" => Some(tig_algorithms::vehicle_routing::c002_a943::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a944")]
                                        // "c002_a944" => Some(tig_algorithms::vehicle_routing::c002_a944::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a945")]
                                        // "c002_a945" => Some(tig_algorithms::vehicle_routing::c002_a945::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a946")]
                                        // "c002_a946" => Some(tig_algorithms::vehicle_routing::c002_a946::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a947")]
                                        // "c002_a947" => Some(tig_algorithms::vehicle_routing::c002_a947::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a948")]
                                        // "c002_a948" => Some(tig_algorithms::vehicle_routing::c002_a948::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a949")]
                                        // "c002_a949" => Some(tig_algorithms::vehicle_routing::c002_a949::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a950")]
                                        // "c002_a950" => Some(tig_algorithms::vehicle_routing::c002_a950::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a951")]
                                        // "c002_a951" => Some(tig_algorithms::vehicle_routing::c002_a951::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a952")]
                                        // "c002_a952" => Some(tig_algorithms::vehicle_routing::c002_a952::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a953")]
                                        // "c002_a953" => Some(tig_algorithms::vehicle_routing::c002_a953::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a954")]
                                        // "c002_a954" => Some(tig_algorithms::vehicle_routing::c002_a954::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a955")]
                                        // "c002_a955" => Some(tig_algorithms::vehicle_routing::c002_a955::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a956")]
                                        // "c002_a956" => Some(tig_algorithms::vehicle_routing::c002_a956::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a957")]
                                        // "c002_a957" => Some(tig_algorithms::vehicle_routing::c002_a957::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a958")]
                                        // "c002_a958" => Some(tig_algorithms::vehicle_routing::c002_a958::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a959")]
                                        // "c002_a959" => Some(tig_algorithms::vehicle_routing::c002_a959::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a960")]
                                        // "c002_a960" => Some(tig_algorithms::vehicle_routing::c002_a960::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a961")]
                                        // "c002_a961" => Some(tig_algorithms::vehicle_routing::c002_a961::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a962")]
                                        // "c002_a962" => Some(tig_algorithms::vehicle_routing::c002_a962::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a963")]
                                        // "c002_a963" => Some(tig_algorithms::vehicle_routing::c002_a963::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a964")]
                                        // "c002_a964" => Some(tig_algorithms::vehicle_routing::c002_a964::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a965")]
                                        // "c002_a965" => Some(tig_algorithms::vehicle_routing::c002_a965::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a966")]
                                        // "c002_a966" => Some(tig_algorithms::vehicle_routing::c002_a966::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a967")]
                                        // "c002_a967" => Some(tig_algorithms::vehicle_routing::c002_a967::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a968")]
                                        // "c002_a968" => Some(tig_algorithms::vehicle_routing::c002_a968::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a969")]
                                        // "c002_a969" => Some(tig_algorithms::vehicle_routing::c002_a969::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a970")]
                                        // "c002_a970" => Some(tig_algorithms::vehicle_routing::c002_a970::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a971")]
                                        // "c002_a971" => Some(tig_algorithms::vehicle_routing::c002_a971::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a972")]
                                        // "c002_a972" => Some(tig_algorithms::vehicle_routing::c002_a972::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a973")]
                                        // "c002_a973" => Some(tig_algorithms::vehicle_routing::c002_a973::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a974")]
                                        // "c002_a974" => Some(tig_algorithms::vehicle_routing::c002_a974::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a975")]
                                        // "c002_a975" => Some(tig_algorithms::vehicle_routing::c002_a975::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a976")]
                                        // "c002_a976" => Some(tig_algorithms::vehicle_routing::c002_a976::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a977")]
                                        // "c002_a977" => Some(tig_algorithms::vehicle_routing::c002_a977::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a978")]
                                        // "c002_a978" => Some(tig_algorithms::vehicle_routing::c002_a978::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a979")]
                                        // "c002_a979" => Some(tig_algorithms::vehicle_routing::c002_a979::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a980")]
                                        // "c002_a980" => Some(tig_algorithms::vehicle_routing::c002_a980::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a981")]
                                        // "c002_a981" => Some(tig_algorithms::vehicle_routing::c002_a981::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a982")]
                                        // "c002_a982" => Some(tig_algorithms::vehicle_routing::c002_a982::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a983")]
                                        // "c002_a983" => Some(tig_algorithms::vehicle_routing::c002_a983::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a984")]
                                        // "c002_a984" => Some(tig_algorithms::vehicle_routing::c002_a984::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a985")]
                                        // "c002_a985" => Some(tig_algorithms::vehicle_routing::c002_a985::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a986")]
                                        // "c002_a986" => Some(tig_algorithms::vehicle_routing::c002_a986::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a987")]
                                        // "c002_a987" => Some(tig_algorithms::vehicle_routing::c002_a987::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a988")]
                                        // "c002_a988" => Some(tig_algorithms::vehicle_routing::c002_a988::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a989")]
                                        // "c002_a989" => Some(tig_algorithms::vehicle_routing::c002_a989::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a990")]
                                        // "c002_a990" => Some(tig_algorithms::vehicle_routing::c002_a990::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a991")]
                                        // "c002_a991" => Some(tig_algorithms::vehicle_routing::c002_a991::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a992")]
                                        // "c002_a992" => Some(tig_algorithms::vehicle_routing::c002_a992::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a993")]
                                        // "c002_a993" => Some(tig_algorithms::vehicle_routing::c002_a993::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a994")]
                                        // "c002_a994" => Some(tig_algorithms::vehicle_routing::c002_a994::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a995")]
                                        // "c002_a995" => Some(tig_algorithms::vehicle_routing::c002_a995::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a996")]
                                        // "c002_a996" => Some(tig_algorithms::vehicle_routing::c002_a996::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a997")]
                                        // "c002_a997" => Some(tig_algorithms::vehicle_routing::c002_a997::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a998")]
                                        // "c002_a998" => Some(tig_algorithms::vehicle_routing::c002_a998::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c002_a999")]
                                        // "c002_a999" => Some(tig_algorithms::vehicle_routing::c002_a999::solve_challenge as SolveChallengeFn),

                                        _ => Option::<SolveChallengeFn>::None
                                    }
                                {
                                    let seed = job.settings.calc_seed(nonce);
                                    let challenge = tig_challenges::vehicle_routing::Challenge::generate_instance_from_vec(seed, &job.settings.difficulty).unwrap();
                                    run_wasm = false;
                                    if let Ok(Some(solution)) = solve_challenge(&challenge) {
                                        run_wasm =
                                            challenge.verify_solution(&solution).is_ok();
                                    }
                                }
                            }
                            "c003" => {
                                type SolveChallengeFn = fn(&tig_challenges::knapsack::Challenge) -> anyhow::Result<Option<tig_challenges::knapsack::Solution>>;
                                if let Some(solve_challenge) =
                                    match job.settings.algorithm_id.as_str() {
                                        // #[cfg(feature = "c003_a001")]
                                        // "c003_a001" => Some(tig_algorithms::knapsack::c003_a001::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a002")]
                                        // "c003_a002" => Some(tig_algorithms::knapsack::c003_a002::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a003")]
                                        // "c003_a003" => Some(tig_algorithms::knapsack::c003_a003::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a004")]
                                        // "c003_a004" => Some(tig_algorithms::knapsack::c003_a004::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a005")]
                                        // "c003_a005" => Some(tig_algorithms::knapsack::c003_a005::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a006")]
                                        // "c003_a006" => Some(tig_algorithms::knapsack::c003_a006::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a007")]
                                        // "c003_a007" => Some(tig_algorithms::knapsack::c003_a007::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a008")]
                                        // "c003_a008" => Some(tig_algorithms::knapsack::c003_a008::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a009")]
                                        // "c003_a009" => Some(tig_algorithms::knapsack::c003_a009::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a010")]
                                        // "c003_a010" => Some(tig_algorithms::knapsack::c003_a010::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a011")]
                                        // "c003_a011" => Some(tig_algorithms::knapsack::c003_a011::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a012")]
                                        // "c003_a012" => Some(tig_algorithms::knapsack::c003_a012::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a013")]
                                        // "c003_a013" => Some(tig_algorithms::knapsack::c003_a013::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a014")]
                                        // "c003_a014" => Some(tig_algorithms::knapsack::c003_a014::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a015")]
                                        // "c003_a015" => Some(tig_algorithms::knapsack::c003_a015::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a016")]
                                        // "c003_a016" => Some(tig_algorithms::knapsack::c003_a016::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a017")]
                                        // "c003_a017" => Some(tig_algorithms::knapsack::c003_a017::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a018")]
                                        // "c003_a018" => Some(tig_algorithms::knapsack::c003_a018::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a019")]
                                        // "c003_a019" => Some(tig_algorithms::knapsack::c003_a019::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a020")]
                                        // "c003_a020" => Some(tig_algorithms::knapsack::c003_a020::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a021")]
                                        // "c003_a021" => Some(tig_algorithms::knapsack::c003_a021::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a022")]
                                        // "c003_a022" => Some(tig_algorithms::knapsack::c003_a022::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a023")]
                                        // "c003_a023" => Some(tig_algorithms::knapsack::c003_a023::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a024")]
                                        // "c003_a024" => Some(tig_algorithms::knapsack::c003_a024::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a025")]
                                        // "c003_a025" => Some(tig_algorithms::knapsack::c003_a025::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a026")]
                                        // "c003_a026" => Some(tig_algorithms::knapsack::c003_a026::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a027")]
                                        // "c003_a027" => Some(tig_algorithms::knapsack::c003_a027::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a028")]
                                        // "c003_a028" => Some(tig_algorithms::knapsack::c003_a028::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a029")]
                                        // "c003_a029" => Some(tig_algorithms::knapsack::c003_a029::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a030")]
                                        // "c003_a030" => Some(tig_algorithms::knapsack::c003_a030::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a031")]
                                        // "c003_a031" => Some(tig_algorithms::knapsack::c003_a031::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a032")]
                                        // "c003_a032" => Some(tig_algorithms::knapsack::c003_a032::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a033")]
                                        // "c003_a033" => Some(tig_algorithms::knapsack::c003_a033::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a034")]
                                        // "c003_a034" => Some(tig_algorithms::knapsack::c003_a034::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a035")]
                                        // "c003_a035" => Some(tig_algorithms::knapsack::c003_a035::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a036")]
                                        // "c003_a036" => Some(tig_algorithms::knapsack::c003_a036::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a037")]
                                        // "c003_a037" => Some(tig_algorithms::knapsack::c003_a037::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a038")]
                                        // "c003_a038" => Some(tig_algorithms::knapsack::c003_a038::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a039")]
                                        // "c003_a039" => Some(tig_algorithms::knapsack::c003_a039::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a040")]
                                        // "c003_a040" => Some(tig_algorithms::knapsack::c003_a040::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a041")]
                                        // "c003_a041" => Some(tig_algorithms::knapsack::c003_a041::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a042")]
                                        // "c003_a042" => Some(tig_algorithms::knapsack::c003_a042::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a043")]
                                        // "c003_a043" => Some(tig_algorithms::knapsack::c003_a043::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a044")]
                                        // "c003_a044" => Some(tig_algorithms::knapsack::c003_a044::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a045")]
                                        // "c003_a045" => Some(tig_algorithms::knapsack::c003_a045::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a046")]
                                        // "c003_a046" => Some(tig_algorithms::knapsack::c003_a046::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a047")]
                                        // "c003_a047" => Some(tig_algorithms::knapsack::c003_a047::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a048")]
                                        // "c003_a048" => Some(tig_algorithms::knapsack::c003_a048::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a049")]
                                        // "c003_a049" => Some(tig_algorithms::knapsack::c003_a049::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a050")]
                                        // "c003_a050" => Some(tig_algorithms::knapsack::c003_a050::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a051")]
                                        // "c003_a051" => Some(tig_algorithms::knapsack::c003_a051::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a052")]
                                        // "c003_a052" => Some(tig_algorithms::knapsack::c003_a052::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a053")]
                                        // "c003_a053" => Some(tig_algorithms::knapsack::c003_a053::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a054")]
                                        // "c003_a054" => Some(tig_algorithms::knapsack::c003_a054::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a055")]
                                        // "c003_a055" => Some(tig_algorithms::knapsack::c003_a055::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a056")]
                                        // "c003_a056" => Some(tig_algorithms::knapsack::c003_a056::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a057")]
                                        // "c003_a057" => Some(tig_algorithms::knapsack::c003_a057::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a058")]
                                        // "c003_a058" => Some(tig_algorithms::knapsack::c003_a058::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a059")]
                                        // "c003_a059" => Some(tig_algorithms::knapsack::c003_a059::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a060")]
                                        // "c003_a060" => Some(tig_algorithms::knapsack::c003_a060::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a061")]
                                        // "c003_a061" => Some(tig_algorithms::knapsack::c003_a061::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a062")]
                                        // "c003_a062" => Some(tig_algorithms::knapsack::c003_a062::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a063")]
                                        // "c003_a063" => Some(tig_algorithms::knapsack::c003_a063::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a064")]
                                        // "c003_a064" => Some(tig_algorithms::knapsack::c003_a064::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a065")]
                                        // "c003_a065" => Some(tig_algorithms::knapsack::c003_a065::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a066")]
                                        // "c003_a066" => Some(tig_algorithms::knapsack::c003_a066::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a067")]
                                        // "c003_a067" => Some(tig_algorithms::knapsack::c003_a067::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a068")]
                                        // "c003_a068" => Some(tig_algorithms::knapsack::c003_a068::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a069")]
                                        // "c003_a069" => Some(tig_algorithms::knapsack::c003_a069::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a070")]
                                        // "c003_a070" => Some(tig_algorithms::knapsack::c003_a070::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a071")]
                                        // "c003_a071" => Some(tig_algorithms::knapsack::c003_a071::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a072")]
                                        // "c003_a072" => Some(tig_algorithms::knapsack::c003_a072::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a073")]
                                        // "c003_a073" => Some(tig_algorithms::knapsack::c003_a073::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a074")]
                                        // "c003_a074" => Some(tig_algorithms::knapsack::c003_a074::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a075")]
                                        // "c003_a075" => Some(tig_algorithms::knapsack::c003_a075::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a076")]
                                        // "c003_a076" => Some(tig_algorithms::knapsack::c003_a076::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a077")]
                                        // "c003_a077" => Some(tig_algorithms::knapsack::c003_a077::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a078")]
                                        // "c003_a078" => Some(tig_algorithms::knapsack::c003_a078::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a079")]
                                        // "c003_a079" => Some(tig_algorithms::knapsack::c003_a079::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a080")]
                                        // "c003_a080" => Some(tig_algorithms::knapsack::c003_a080::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a081")]
                                        // "c003_a081" => Some(tig_algorithms::knapsack::c003_a081::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a082")]
                                        // "c003_a082" => Some(tig_algorithms::knapsack::c003_a082::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a083")]
                                        // "c003_a083" => Some(tig_algorithms::knapsack::c003_a083::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a084")]
                                        // "c003_a084" => Some(tig_algorithms::knapsack::c003_a084::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a085")]
                                        // "c003_a085" => Some(tig_algorithms::knapsack::c003_a085::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a086")]
                                        // "c003_a086" => Some(tig_algorithms::knapsack::c003_a086::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a087")]
                                        // "c003_a087" => Some(tig_algorithms::knapsack::c003_a087::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a088")]
                                        // "c003_a088" => Some(tig_algorithms::knapsack::c003_a088::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a089")]
                                        // "c003_a089" => Some(tig_algorithms::knapsack::c003_a089::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a090")]
                                        // "c003_a090" => Some(tig_algorithms::knapsack::c003_a090::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a091")]
                                        // "c003_a091" => Some(tig_algorithms::knapsack::c003_a091::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a092")]
                                        // "c003_a092" => Some(tig_algorithms::knapsack::c003_a092::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a093")]
                                        // "c003_a093" => Some(tig_algorithms::knapsack::c003_a093::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a094")]
                                        // "c003_a094" => Some(tig_algorithms::knapsack::c003_a094::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a095")]
                                        // "c003_a095" => Some(tig_algorithms::knapsack::c003_a095::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a096")]
                                        // "c003_a096" => Some(tig_algorithms::knapsack::c003_a096::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a097")]
                                        // "c003_a097" => Some(tig_algorithms::knapsack::c003_a097::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a098")]
                                        // "c003_a098" => Some(tig_algorithms::knapsack::c003_a098::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a099")]
                                        // "c003_a099" => Some(tig_algorithms::knapsack::c003_a099::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a100")]
                                        // "c003_a100" => Some(tig_algorithms::knapsack::c003_a100::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a101")]
                                        // "c003_a101" => Some(tig_algorithms::knapsack::c003_a101::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a102")]
                                        // "c003_a102" => Some(tig_algorithms::knapsack::c003_a102::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a103")]
                                        // "c003_a103" => Some(tig_algorithms::knapsack::c003_a103::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a104")]
                                        // "c003_a104" => Some(tig_algorithms::knapsack::c003_a104::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a105")]
                                        // "c003_a105" => Some(tig_algorithms::knapsack::c003_a105::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a106")]
                                        // "c003_a106" => Some(tig_algorithms::knapsack::c003_a106::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a107")]
                                        // "c003_a107" => Some(tig_algorithms::knapsack::c003_a107::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a108")]
                                        // "c003_a108" => Some(tig_algorithms::knapsack::c003_a108::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a109")]
                                        // "c003_a109" => Some(tig_algorithms::knapsack::c003_a109::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a110")]
                                        // "c003_a110" => Some(tig_algorithms::knapsack::c003_a110::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a111")]
                                        // "c003_a111" => Some(tig_algorithms::knapsack::c003_a111::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a112")]
                                        // "c003_a112" => Some(tig_algorithms::knapsack::c003_a112::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a113")]
                                        // "c003_a113" => Some(tig_algorithms::knapsack::c003_a113::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a114")]
                                        // "c003_a114" => Some(tig_algorithms::knapsack::c003_a114::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a115")]
                                        // "c003_a115" => Some(tig_algorithms::knapsack::c003_a115::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a116")]
                                        // "c003_a116" => Some(tig_algorithms::knapsack::c003_a116::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a117")]
                                        // "c003_a117" => Some(tig_algorithms::knapsack::c003_a117::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a118")]
                                        // "c003_a118" => Some(tig_algorithms::knapsack::c003_a118::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a119")]
                                        // "c003_a119" => Some(tig_algorithms::knapsack::c003_a119::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a120")]
                                        // "c003_a120" => Some(tig_algorithms::knapsack::c003_a120::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a121")]
                                        // "c003_a121" => Some(tig_algorithms::knapsack::c003_a121::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a122")]
                                        // "c003_a122" => Some(tig_algorithms::knapsack::c003_a122::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a123")]
                                        // "c003_a123" => Some(tig_algorithms::knapsack::c003_a123::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a124")]
                                        // "c003_a124" => Some(tig_algorithms::knapsack::c003_a124::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a125")]
                                        // "c003_a125" => Some(tig_algorithms::knapsack::c003_a125::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a126")]
                                        // "c003_a126" => Some(tig_algorithms::knapsack::c003_a126::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a127")]
                                        // "c003_a127" => Some(tig_algorithms::knapsack::c003_a127::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a128")]
                                        // "c003_a128" => Some(tig_algorithms::knapsack::c003_a128::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a129")]
                                        // "c003_a129" => Some(tig_algorithms::knapsack::c003_a129::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a130")]
                                        // "c003_a130" => Some(tig_algorithms::knapsack::c003_a130::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a131")]
                                        // "c003_a131" => Some(tig_algorithms::knapsack::c003_a131::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a132")]
                                        // "c003_a132" => Some(tig_algorithms::knapsack::c003_a132::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a133")]
                                        // "c003_a133" => Some(tig_algorithms::knapsack::c003_a133::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a134")]
                                        // "c003_a134" => Some(tig_algorithms::knapsack::c003_a134::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a135")]
                                        // "c003_a135" => Some(tig_algorithms::knapsack::c003_a135::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a136")]
                                        // "c003_a136" => Some(tig_algorithms::knapsack::c003_a136::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a137")]
                                        // "c003_a137" => Some(tig_algorithms::knapsack::c003_a137::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a138")]
                                        // "c003_a138" => Some(tig_algorithms::knapsack::c003_a138::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a139")]
                                        // "c003_a139" => Some(tig_algorithms::knapsack::c003_a139::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a140")]
                                        // "c003_a140" => Some(tig_algorithms::knapsack::c003_a140::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a141")]
                                        // "c003_a141" => Some(tig_algorithms::knapsack::c003_a141::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a142")]
                                        // "c003_a142" => Some(tig_algorithms::knapsack::c003_a142::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a143")]
                                        // "c003_a143" => Some(tig_algorithms::knapsack::c003_a143::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a144")]
                                        // "c003_a144" => Some(tig_algorithms::knapsack::c003_a144::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a145")]
                                        // "c003_a145" => Some(tig_algorithms::knapsack::c003_a145::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a146")]
                                        // "c003_a146" => Some(tig_algorithms::knapsack::c003_a146::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a147")]
                                        // "c003_a147" => Some(tig_algorithms::knapsack::c003_a147::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a148")]
                                        // "c003_a148" => Some(tig_algorithms::knapsack::c003_a148::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a149")]
                                        // "c003_a149" => Some(tig_algorithms::knapsack::c003_a149::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a150")]
                                        // "c003_a150" => Some(tig_algorithms::knapsack::c003_a150::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a151")]
                                        // "c003_a151" => Some(tig_algorithms::knapsack::c003_a151::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a152")]
                                        // "c003_a152" => Some(tig_algorithms::knapsack::c003_a152::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a153")]
                                        // "c003_a153" => Some(tig_algorithms::knapsack::c003_a153::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a154")]
                                        // "c003_a154" => Some(tig_algorithms::knapsack::c003_a154::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a155")]
                                        // "c003_a155" => Some(tig_algorithms::knapsack::c003_a155::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a156")]
                                        // "c003_a156" => Some(tig_algorithms::knapsack::c003_a156::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a157")]
                                        // "c003_a157" => Some(tig_algorithms::knapsack::c003_a157::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a158")]
                                        // "c003_a158" => Some(tig_algorithms::knapsack::c003_a158::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a159")]
                                        // "c003_a159" => Some(tig_algorithms::knapsack::c003_a159::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a160")]
                                        // "c003_a160" => Some(tig_algorithms::knapsack::c003_a160::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a161")]
                                        // "c003_a161" => Some(tig_algorithms::knapsack::c003_a161::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a162")]
                                        // "c003_a162" => Some(tig_algorithms::knapsack::c003_a162::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a163")]
                                        // "c003_a163" => Some(tig_algorithms::knapsack::c003_a163::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a164")]
                                        // "c003_a164" => Some(tig_algorithms::knapsack::c003_a164::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a165")]
                                        // "c003_a165" => Some(tig_algorithms::knapsack::c003_a165::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a166")]
                                        // "c003_a166" => Some(tig_algorithms::knapsack::c003_a166::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a167")]
                                        // "c003_a167" => Some(tig_algorithms::knapsack::c003_a167::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a168")]
                                        // "c003_a168" => Some(tig_algorithms::knapsack::c003_a168::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a169")]
                                        // "c003_a169" => Some(tig_algorithms::knapsack::c003_a169::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a170")]
                                        // "c003_a170" => Some(tig_algorithms::knapsack::c003_a170::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a171")]
                                        // "c003_a171" => Some(tig_algorithms::knapsack::c003_a171::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a172")]
                                        // "c003_a172" => Some(tig_algorithms::knapsack::c003_a172::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a173")]
                                        // "c003_a173" => Some(tig_algorithms::knapsack::c003_a173::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a174")]
                                        // "c003_a174" => Some(tig_algorithms::knapsack::c003_a174::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a175")]
                                        // "c003_a175" => Some(tig_algorithms::knapsack::c003_a175::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a176")]
                                        // "c003_a176" => Some(tig_algorithms::knapsack::c003_a176::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a177")]
                                        // "c003_a177" => Some(tig_algorithms::knapsack::c003_a177::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a178")]
                                        // "c003_a178" => Some(tig_algorithms::knapsack::c003_a178::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a179")]
                                        // "c003_a179" => Some(tig_algorithms::knapsack::c003_a179::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a180")]
                                        // "c003_a180" => Some(tig_algorithms::knapsack::c003_a180::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a181")]
                                        // "c003_a181" => Some(tig_algorithms::knapsack::c003_a181::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a182")]
                                        // "c003_a182" => Some(tig_algorithms::knapsack::c003_a182::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a183")]
                                        // "c003_a183" => Some(tig_algorithms::knapsack::c003_a183::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a184")]
                                        // "c003_a184" => Some(tig_algorithms::knapsack::c003_a184::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a185")]
                                        // "c003_a185" => Some(tig_algorithms::knapsack::c003_a185::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a186")]
                                        // "c003_a186" => Some(tig_algorithms::knapsack::c003_a186::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a187")]
                                        // "c003_a187" => Some(tig_algorithms::knapsack::c003_a187::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a188")]
                                        // "c003_a188" => Some(tig_algorithms::knapsack::c003_a188::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a189")]
                                        // "c003_a189" => Some(tig_algorithms::knapsack::c003_a189::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a190")]
                                        // "c003_a190" => Some(tig_algorithms::knapsack::c003_a190::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a191")]
                                        // "c003_a191" => Some(tig_algorithms::knapsack::c003_a191::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a192")]
                                        // "c003_a192" => Some(tig_algorithms::knapsack::c003_a192::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a193")]
                                        // "c003_a193" => Some(tig_algorithms::knapsack::c003_a193::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a194")]
                                        // "c003_a194" => Some(tig_algorithms::knapsack::c003_a194::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a195")]
                                        // "c003_a195" => Some(tig_algorithms::knapsack::c003_a195::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a196")]
                                        // "c003_a196" => Some(tig_algorithms::knapsack::c003_a196::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a197")]
                                        // "c003_a197" => Some(tig_algorithms::knapsack::c003_a197::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a198")]
                                        // "c003_a198" => Some(tig_algorithms::knapsack::c003_a198::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a199")]
                                        // "c003_a199" => Some(tig_algorithms::knapsack::c003_a199::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a200")]
                                        // "c003_a200" => Some(tig_algorithms::knapsack::c003_a200::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a201")]
                                        // "c003_a201" => Some(tig_algorithms::knapsack::c003_a201::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a202")]
                                        // "c003_a202" => Some(tig_algorithms::knapsack::c003_a202::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a203")]
                                        // "c003_a203" => Some(tig_algorithms::knapsack::c003_a203::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a204")]
                                        // "c003_a204" => Some(tig_algorithms::knapsack::c003_a204::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a205")]
                                        // "c003_a205" => Some(tig_algorithms::knapsack::c003_a205::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a206")]
                                        // "c003_a206" => Some(tig_algorithms::knapsack::c003_a206::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a207")]
                                        // "c003_a207" => Some(tig_algorithms::knapsack::c003_a207::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a208")]
                                        // "c003_a208" => Some(tig_algorithms::knapsack::c003_a208::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a209")]
                                        // "c003_a209" => Some(tig_algorithms::knapsack::c003_a209::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a210")]
                                        // "c003_a210" => Some(tig_algorithms::knapsack::c003_a210::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a211")]
                                        // "c003_a211" => Some(tig_algorithms::knapsack::c003_a211::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a212")]
                                        // "c003_a212" => Some(tig_algorithms::knapsack::c003_a212::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a213")]
                                        // "c003_a213" => Some(tig_algorithms::knapsack::c003_a213::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a214")]
                                        // "c003_a214" => Some(tig_algorithms::knapsack::c003_a214::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a215")]
                                        // "c003_a215" => Some(tig_algorithms::knapsack::c003_a215::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a216")]
                                        // "c003_a216" => Some(tig_algorithms::knapsack::c003_a216::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a217")]
                                        // "c003_a217" => Some(tig_algorithms::knapsack::c003_a217::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a218")]
                                        // "c003_a218" => Some(tig_algorithms::knapsack::c003_a218::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a219")]
                                        // "c003_a219" => Some(tig_algorithms::knapsack::c003_a219::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a220")]
                                        // "c003_a220" => Some(tig_algorithms::knapsack::c003_a220::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a221")]
                                        // "c003_a221" => Some(tig_algorithms::knapsack::c003_a221::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a222")]
                                        // "c003_a222" => Some(tig_algorithms::knapsack::c003_a222::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a223")]
                                        // "c003_a223" => Some(tig_algorithms::knapsack::c003_a223::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a224")]
                                        // "c003_a224" => Some(tig_algorithms::knapsack::c003_a224::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a225")]
                                        // "c003_a225" => Some(tig_algorithms::knapsack::c003_a225::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a226")]
                                        // "c003_a226" => Some(tig_algorithms::knapsack::c003_a226::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a227")]
                                        // "c003_a227" => Some(tig_algorithms::knapsack::c003_a227::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a228")]
                                        // "c003_a228" => Some(tig_algorithms::knapsack::c003_a228::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a229")]
                                        // "c003_a229" => Some(tig_algorithms::knapsack::c003_a229::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a230")]
                                        // "c003_a230" => Some(tig_algorithms::knapsack::c003_a230::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a231")]
                                        // "c003_a231" => Some(tig_algorithms::knapsack::c003_a231::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a232")]
                                        // "c003_a232" => Some(tig_algorithms::knapsack::c003_a232::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a233")]
                                        // "c003_a233" => Some(tig_algorithms::knapsack::c003_a233::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a234")]
                                        // "c003_a234" => Some(tig_algorithms::knapsack::c003_a234::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a235")]
                                        // "c003_a235" => Some(tig_algorithms::knapsack::c003_a235::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a236")]
                                        // "c003_a236" => Some(tig_algorithms::knapsack::c003_a236::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a237")]
                                        // "c003_a237" => Some(tig_algorithms::knapsack::c003_a237::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a238")]
                                        // "c003_a238" => Some(tig_algorithms::knapsack::c003_a238::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a239")]
                                        // "c003_a239" => Some(tig_algorithms::knapsack::c003_a239::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a240")]
                                        // "c003_a240" => Some(tig_algorithms::knapsack::c003_a240::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a241")]
                                        // "c003_a241" => Some(tig_algorithms::knapsack::c003_a241::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a242")]
                                        // "c003_a242" => Some(tig_algorithms::knapsack::c003_a242::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a243")]
                                        // "c003_a243" => Some(tig_algorithms::knapsack::c003_a243::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a244")]
                                        // "c003_a244" => Some(tig_algorithms::knapsack::c003_a244::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a245")]
                                        // "c003_a245" => Some(tig_algorithms::knapsack::c003_a245::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a246")]
                                        // "c003_a246" => Some(tig_algorithms::knapsack::c003_a246::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a247")]
                                        // "c003_a247" => Some(tig_algorithms::knapsack::c003_a247::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a248")]
                                        // "c003_a248" => Some(tig_algorithms::knapsack::c003_a248::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a249")]
                                        // "c003_a249" => Some(tig_algorithms::knapsack::c003_a249::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a250")]
                                        // "c003_a250" => Some(tig_algorithms::knapsack::c003_a250::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a251")]
                                        // "c003_a251" => Some(tig_algorithms::knapsack::c003_a251::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a252")]
                                        // "c003_a252" => Some(tig_algorithms::knapsack::c003_a252::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a253")]
                                        // "c003_a253" => Some(tig_algorithms::knapsack::c003_a253::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a254")]
                                        // "c003_a254" => Some(tig_algorithms::knapsack::c003_a254::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a255")]
                                        // "c003_a255" => Some(tig_algorithms::knapsack::c003_a255::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a256")]
                                        // "c003_a256" => Some(tig_algorithms::knapsack::c003_a256::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a257")]
                                        // "c003_a257" => Some(tig_algorithms::knapsack::c003_a257::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a258")]
                                        // "c003_a258" => Some(tig_algorithms::knapsack::c003_a258::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a259")]
                                        // "c003_a259" => Some(tig_algorithms::knapsack::c003_a259::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a260")]
                                        // "c003_a260" => Some(tig_algorithms::knapsack::c003_a260::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a261")]
                                        // "c003_a261" => Some(tig_algorithms::knapsack::c003_a261::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a262")]
                                        // "c003_a262" => Some(tig_algorithms::knapsack::c003_a262::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a263")]
                                        // "c003_a263" => Some(tig_algorithms::knapsack::c003_a263::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a264")]
                                        // "c003_a264" => Some(tig_algorithms::knapsack::c003_a264::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a265")]
                                        // "c003_a265" => Some(tig_algorithms::knapsack::c003_a265::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a266")]
                                        // "c003_a266" => Some(tig_algorithms::knapsack::c003_a266::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a267")]
                                        // "c003_a267" => Some(tig_algorithms::knapsack::c003_a267::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a268")]
                                        // "c003_a268" => Some(tig_algorithms::knapsack::c003_a268::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a269")]
                                        // "c003_a269" => Some(tig_algorithms::knapsack::c003_a269::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a270")]
                                        // "c003_a270" => Some(tig_algorithms::knapsack::c003_a270::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a271")]
                                        // "c003_a271" => Some(tig_algorithms::knapsack::c003_a271::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a272")]
                                        // "c003_a272" => Some(tig_algorithms::knapsack::c003_a272::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a273")]
                                        // "c003_a273" => Some(tig_algorithms::knapsack::c003_a273::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a274")]
                                        // "c003_a274" => Some(tig_algorithms::knapsack::c003_a274::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a275")]
                                        // "c003_a275" => Some(tig_algorithms::knapsack::c003_a275::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a276")]
                                        // "c003_a276" => Some(tig_algorithms::knapsack::c003_a276::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a277")]
                                        // "c003_a277" => Some(tig_algorithms::knapsack::c003_a277::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a278")]
                                        // "c003_a278" => Some(tig_algorithms::knapsack::c003_a278::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a279")]
                                        // "c003_a279" => Some(tig_algorithms::knapsack::c003_a279::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a280")]
                                        // "c003_a280" => Some(tig_algorithms::knapsack::c003_a280::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a281")]
                                        // "c003_a281" => Some(tig_algorithms::knapsack::c003_a281::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a282")]
                                        // "c003_a282" => Some(tig_algorithms::knapsack::c003_a282::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a283")]
                                        // "c003_a283" => Some(tig_algorithms::knapsack::c003_a283::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a284")]
                                        // "c003_a284" => Some(tig_algorithms::knapsack::c003_a284::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a285")]
                                        // "c003_a285" => Some(tig_algorithms::knapsack::c003_a285::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a286")]
                                        // "c003_a286" => Some(tig_algorithms::knapsack::c003_a286::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a287")]
                                        // "c003_a287" => Some(tig_algorithms::knapsack::c003_a287::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a288")]
                                        // "c003_a288" => Some(tig_algorithms::knapsack::c003_a288::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a289")]
                                        // "c003_a289" => Some(tig_algorithms::knapsack::c003_a289::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a290")]
                                        // "c003_a290" => Some(tig_algorithms::knapsack::c003_a290::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a291")]
                                        // "c003_a291" => Some(tig_algorithms::knapsack::c003_a291::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a292")]
                                        // "c003_a292" => Some(tig_algorithms::knapsack::c003_a292::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a293")]
                                        // "c003_a293" => Some(tig_algorithms::knapsack::c003_a293::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a294")]
                                        // "c003_a294" => Some(tig_algorithms::knapsack::c003_a294::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a295")]
                                        // "c003_a295" => Some(tig_algorithms::knapsack::c003_a295::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a296")]
                                        // "c003_a296" => Some(tig_algorithms::knapsack::c003_a296::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a297")]
                                        // "c003_a297" => Some(tig_algorithms::knapsack::c003_a297::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a298")]
                                        // "c003_a298" => Some(tig_algorithms::knapsack::c003_a298::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a299")]
                                        // "c003_a299" => Some(tig_algorithms::knapsack::c003_a299::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a300")]
                                        // "c003_a300" => Some(tig_algorithms::knapsack::c003_a300::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a301")]
                                        // "c003_a301" => Some(tig_algorithms::knapsack::c003_a301::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a302")]
                                        // "c003_a302" => Some(tig_algorithms::knapsack::c003_a302::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a303")]
                                        // "c003_a303" => Some(tig_algorithms::knapsack::c003_a303::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a304")]
                                        // "c003_a304" => Some(tig_algorithms::knapsack::c003_a304::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a305")]
                                        // "c003_a305" => Some(tig_algorithms::knapsack::c003_a305::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a306")]
                                        // "c003_a306" => Some(tig_algorithms::knapsack::c003_a306::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a307")]
                                        // "c003_a307" => Some(tig_algorithms::knapsack::c003_a307::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a308")]
                                        // "c003_a308" => Some(tig_algorithms::knapsack::c003_a308::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a309")]
                                        // "c003_a309" => Some(tig_algorithms::knapsack::c003_a309::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a310")]
                                        // "c003_a310" => Some(tig_algorithms::knapsack::c003_a310::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a311")]
                                        // "c003_a311" => Some(tig_algorithms::knapsack::c003_a311::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a312")]
                                        // "c003_a312" => Some(tig_algorithms::knapsack::c003_a312::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a313")]
                                        // "c003_a313" => Some(tig_algorithms::knapsack::c003_a313::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a314")]
                                        // "c003_a314" => Some(tig_algorithms::knapsack::c003_a314::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a315")]
                                        // "c003_a315" => Some(tig_algorithms::knapsack::c003_a315::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a316")]
                                        // "c003_a316" => Some(tig_algorithms::knapsack::c003_a316::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a317")]
                                        // "c003_a317" => Some(tig_algorithms::knapsack::c003_a317::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a318")]
                                        // "c003_a318" => Some(tig_algorithms::knapsack::c003_a318::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a319")]
                                        // "c003_a319" => Some(tig_algorithms::knapsack::c003_a319::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a320")]
                                        // "c003_a320" => Some(tig_algorithms::knapsack::c003_a320::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a321")]
                                        // "c003_a321" => Some(tig_algorithms::knapsack::c003_a321::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a322")]
                                        // "c003_a322" => Some(tig_algorithms::knapsack::c003_a322::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a323")]
                                        // "c003_a323" => Some(tig_algorithms::knapsack::c003_a323::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a324")]
                                        // "c003_a324" => Some(tig_algorithms::knapsack::c003_a324::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a325")]
                                        // "c003_a325" => Some(tig_algorithms::knapsack::c003_a325::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a326")]
                                        // "c003_a326" => Some(tig_algorithms::knapsack::c003_a326::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a327")]
                                        // "c003_a327" => Some(tig_algorithms::knapsack::c003_a327::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a328")]
                                        // "c003_a328" => Some(tig_algorithms::knapsack::c003_a328::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a329")]
                                        // "c003_a329" => Some(tig_algorithms::knapsack::c003_a329::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a330")]
                                        // "c003_a330" => Some(tig_algorithms::knapsack::c003_a330::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a331")]
                                        // "c003_a331" => Some(tig_algorithms::knapsack::c003_a331::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a332")]
                                        // "c003_a332" => Some(tig_algorithms::knapsack::c003_a332::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a333")]
                                        // "c003_a333" => Some(tig_algorithms::knapsack::c003_a333::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a334")]
                                        // "c003_a334" => Some(tig_algorithms::knapsack::c003_a334::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a335")]
                                        // "c003_a335" => Some(tig_algorithms::knapsack::c003_a335::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a336")]
                                        // "c003_a336" => Some(tig_algorithms::knapsack::c003_a336::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a337")]
                                        // "c003_a337" => Some(tig_algorithms::knapsack::c003_a337::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a338")]
                                        // "c003_a338" => Some(tig_algorithms::knapsack::c003_a338::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a339")]
                                        // "c003_a339" => Some(tig_algorithms::knapsack::c003_a339::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a340")]
                                        // "c003_a340" => Some(tig_algorithms::knapsack::c003_a340::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a341")]
                                        // "c003_a341" => Some(tig_algorithms::knapsack::c003_a341::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a342")]
                                        // "c003_a342" => Some(tig_algorithms::knapsack::c003_a342::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a343")]
                                        // "c003_a343" => Some(tig_algorithms::knapsack::c003_a343::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a344")]
                                        // "c003_a344" => Some(tig_algorithms::knapsack::c003_a344::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a345")]
                                        // "c003_a345" => Some(tig_algorithms::knapsack::c003_a345::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a346")]
                                        // "c003_a346" => Some(tig_algorithms::knapsack::c003_a346::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a347")]
                                        // "c003_a347" => Some(tig_algorithms::knapsack::c003_a347::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a348")]
                                        // "c003_a348" => Some(tig_algorithms::knapsack::c003_a348::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a349")]
                                        // "c003_a349" => Some(tig_algorithms::knapsack::c003_a349::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a350")]
                                        // "c003_a350" => Some(tig_algorithms::knapsack::c003_a350::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a351")]
                                        // "c003_a351" => Some(tig_algorithms::knapsack::c003_a351::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a352")]
                                        // "c003_a352" => Some(tig_algorithms::knapsack::c003_a352::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a353")]
                                        // "c003_a353" => Some(tig_algorithms::knapsack::c003_a353::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a354")]
                                        // "c003_a354" => Some(tig_algorithms::knapsack::c003_a354::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a355")]
                                        // "c003_a355" => Some(tig_algorithms::knapsack::c003_a355::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a356")]
                                        // "c003_a356" => Some(tig_algorithms::knapsack::c003_a356::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a357")]
                                        // "c003_a357" => Some(tig_algorithms::knapsack::c003_a357::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a358")]
                                        // "c003_a358" => Some(tig_algorithms::knapsack::c003_a358::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a359")]
                                        // "c003_a359" => Some(tig_algorithms::knapsack::c003_a359::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a360")]
                                        // "c003_a360" => Some(tig_algorithms::knapsack::c003_a360::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a361")]
                                        // "c003_a361" => Some(tig_algorithms::knapsack::c003_a361::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a362")]
                                        // "c003_a362" => Some(tig_algorithms::knapsack::c003_a362::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a363")]
                                        // "c003_a363" => Some(tig_algorithms::knapsack::c003_a363::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a364")]
                                        // "c003_a364" => Some(tig_algorithms::knapsack::c003_a364::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a365")]
                                        // "c003_a365" => Some(tig_algorithms::knapsack::c003_a365::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a366")]
                                        // "c003_a366" => Some(tig_algorithms::knapsack::c003_a366::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a367")]
                                        // "c003_a367" => Some(tig_algorithms::knapsack::c003_a367::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a368")]
                                        // "c003_a368" => Some(tig_algorithms::knapsack::c003_a368::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a369")]
                                        // "c003_a369" => Some(tig_algorithms::knapsack::c003_a369::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a370")]
                                        // "c003_a370" => Some(tig_algorithms::knapsack::c003_a370::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a371")]
                                        // "c003_a371" => Some(tig_algorithms::knapsack::c003_a371::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a372")]
                                        // "c003_a372" => Some(tig_algorithms::knapsack::c003_a372::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a373")]
                                        // "c003_a373" => Some(tig_algorithms::knapsack::c003_a373::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a374")]
                                        // "c003_a374" => Some(tig_algorithms::knapsack::c003_a374::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a375")]
                                        // "c003_a375" => Some(tig_algorithms::knapsack::c003_a375::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a376")]
                                        // "c003_a376" => Some(tig_algorithms::knapsack::c003_a376::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a377")]
                                        // "c003_a377" => Some(tig_algorithms::knapsack::c003_a377::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a378")]
                                        // "c003_a378" => Some(tig_algorithms::knapsack::c003_a378::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a379")]
                                        // "c003_a379" => Some(tig_algorithms::knapsack::c003_a379::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a380")]
                                        // "c003_a380" => Some(tig_algorithms::knapsack::c003_a380::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a381")]
                                        // "c003_a381" => Some(tig_algorithms::knapsack::c003_a381::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a382")]
                                        // "c003_a382" => Some(tig_algorithms::knapsack::c003_a382::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a383")]
                                        // "c003_a383" => Some(tig_algorithms::knapsack::c003_a383::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a384")]
                                        // "c003_a384" => Some(tig_algorithms::knapsack::c003_a384::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a385")]
                                        // "c003_a385" => Some(tig_algorithms::knapsack::c003_a385::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a386")]
                                        // "c003_a386" => Some(tig_algorithms::knapsack::c003_a386::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a387")]
                                        // "c003_a387" => Some(tig_algorithms::knapsack::c003_a387::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a388")]
                                        // "c003_a388" => Some(tig_algorithms::knapsack::c003_a388::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a389")]
                                        // "c003_a389" => Some(tig_algorithms::knapsack::c003_a389::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a390")]
                                        // "c003_a390" => Some(tig_algorithms::knapsack::c003_a390::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a391")]
                                        // "c003_a391" => Some(tig_algorithms::knapsack::c003_a391::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a392")]
                                        // "c003_a392" => Some(tig_algorithms::knapsack::c003_a392::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a393")]
                                        // "c003_a393" => Some(tig_algorithms::knapsack::c003_a393::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a394")]
                                        // "c003_a394" => Some(tig_algorithms::knapsack::c003_a394::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a395")]
                                        // "c003_a395" => Some(tig_algorithms::knapsack::c003_a395::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a396")]
                                        // "c003_a396" => Some(tig_algorithms::knapsack::c003_a396::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a397")]
                                        // "c003_a397" => Some(tig_algorithms::knapsack::c003_a397::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a398")]
                                        // "c003_a398" => Some(tig_algorithms::knapsack::c003_a398::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a399")]
                                        // "c003_a399" => Some(tig_algorithms::knapsack::c003_a399::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a400")]
                                        // "c003_a400" => Some(tig_algorithms::knapsack::c003_a400::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a401")]
                                        // "c003_a401" => Some(tig_algorithms::knapsack::c003_a401::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a402")]
                                        // "c003_a402" => Some(tig_algorithms::knapsack::c003_a402::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a403")]
                                        // "c003_a403" => Some(tig_algorithms::knapsack::c003_a403::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a404")]
                                        // "c003_a404" => Some(tig_algorithms::knapsack::c003_a404::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a405")]
                                        // "c003_a405" => Some(tig_algorithms::knapsack::c003_a405::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a406")]
                                        // "c003_a406" => Some(tig_algorithms::knapsack::c003_a406::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a407")]
                                        // "c003_a407" => Some(tig_algorithms::knapsack::c003_a407::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a408")]
                                        // "c003_a408" => Some(tig_algorithms::knapsack::c003_a408::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a409")]
                                        // "c003_a409" => Some(tig_algorithms::knapsack::c003_a409::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a410")]
                                        // "c003_a410" => Some(tig_algorithms::knapsack::c003_a410::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a411")]
                                        // "c003_a411" => Some(tig_algorithms::knapsack::c003_a411::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a412")]
                                        // "c003_a412" => Some(tig_algorithms::knapsack::c003_a412::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a413")]
                                        // "c003_a413" => Some(tig_algorithms::knapsack::c003_a413::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a414")]
                                        // "c003_a414" => Some(tig_algorithms::knapsack::c003_a414::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a415")]
                                        // "c003_a415" => Some(tig_algorithms::knapsack::c003_a415::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a416")]
                                        // "c003_a416" => Some(tig_algorithms::knapsack::c003_a416::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a417")]
                                        // "c003_a417" => Some(tig_algorithms::knapsack::c003_a417::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a418")]
                                        // "c003_a418" => Some(tig_algorithms::knapsack::c003_a418::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a419")]
                                        // "c003_a419" => Some(tig_algorithms::knapsack::c003_a419::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a420")]
                                        // "c003_a420" => Some(tig_algorithms::knapsack::c003_a420::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a421")]
                                        // "c003_a421" => Some(tig_algorithms::knapsack::c003_a421::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a422")]
                                        // "c003_a422" => Some(tig_algorithms::knapsack::c003_a422::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a423")]
                                        // "c003_a423" => Some(tig_algorithms::knapsack::c003_a423::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a424")]
                                        // "c003_a424" => Some(tig_algorithms::knapsack::c003_a424::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a425")]
                                        // "c003_a425" => Some(tig_algorithms::knapsack::c003_a425::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a426")]
                                        // "c003_a426" => Some(tig_algorithms::knapsack::c003_a426::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a427")]
                                        // "c003_a427" => Some(tig_algorithms::knapsack::c003_a427::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a428")]
                                        // "c003_a428" => Some(tig_algorithms::knapsack::c003_a428::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a429")]
                                        // "c003_a429" => Some(tig_algorithms::knapsack::c003_a429::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a430")]
                                        // "c003_a430" => Some(tig_algorithms::knapsack::c003_a430::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a431")]
                                        // "c003_a431" => Some(tig_algorithms::knapsack::c003_a431::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a432")]
                                        // "c003_a432" => Some(tig_algorithms::knapsack::c003_a432::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a433")]
                                        // "c003_a433" => Some(tig_algorithms::knapsack::c003_a433::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a434")]
                                        // "c003_a434" => Some(tig_algorithms::knapsack::c003_a434::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a435")]
                                        // "c003_a435" => Some(tig_algorithms::knapsack::c003_a435::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a436")]
                                        // "c003_a436" => Some(tig_algorithms::knapsack::c003_a436::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a437")]
                                        // "c003_a437" => Some(tig_algorithms::knapsack::c003_a437::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a438")]
                                        // "c003_a438" => Some(tig_algorithms::knapsack::c003_a438::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a439")]
                                        // "c003_a439" => Some(tig_algorithms::knapsack::c003_a439::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a440")]
                                        // "c003_a440" => Some(tig_algorithms::knapsack::c003_a440::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a441")]
                                        // "c003_a441" => Some(tig_algorithms::knapsack::c003_a441::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a442")]
                                        // "c003_a442" => Some(tig_algorithms::knapsack::c003_a442::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a443")]
                                        // "c003_a443" => Some(tig_algorithms::knapsack::c003_a443::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a444")]
                                        // "c003_a444" => Some(tig_algorithms::knapsack::c003_a444::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a445")]
                                        // "c003_a445" => Some(tig_algorithms::knapsack::c003_a445::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a446")]
                                        // "c003_a446" => Some(tig_algorithms::knapsack::c003_a446::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a447")]
                                        // "c003_a447" => Some(tig_algorithms::knapsack::c003_a447::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a448")]
                                        // "c003_a448" => Some(tig_algorithms::knapsack::c003_a448::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a449")]
                                        // "c003_a449" => Some(tig_algorithms::knapsack::c003_a449::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a450")]
                                        // "c003_a450" => Some(tig_algorithms::knapsack::c003_a450::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a451")]
                                        // "c003_a451" => Some(tig_algorithms::knapsack::c003_a451::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a452")]
                                        // "c003_a452" => Some(tig_algorithms::knapsack::c003_a452::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a453")]
                                        // "c003_a453" => Some(tig_algorithms::knapsack::c003_a453::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a454")]
                                        // "c003_a454" => Some(tig_algorithms::knapsack::c003_a454::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a455")]
                                        // "c003_a455" => Some(tig_algorithms::knapsack::c003_a455::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a456")]
                                        // "c003_a456" => Some(tig_algorithms::knapsack::c003_a456::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a457")]
                                        // "c003_a457" => Some(tig_algorithms::knapsack::c003_a457::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a458")]
                                        // "c003_a458" => Some(tig_algorithms::knapsack::c003_a458::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a459")]
                                        // "c003_a459" => Some(tig_algorithms::knapsack::c003_a459::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a460")]
                                        // "c003_a460" => Some(tig_algorithms::knapsack::c003_a460::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a461")]
                                        // "c003_a461" => Some(tig_algorithms::knapsack::c003_a461::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a462")]
                                        // "c003_a462" => Some(tig_algorithms::knapsack::c003_a462::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a463")]
                                        // "c003_a463" => Some(tig_algorithms::knapsack::c003_a463::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a464")]
                                        // "c003_a464" => Some(tig_algorithms::knapsack::c003_a464::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a465")]
                                        // "c003_a465" => Some(tig_algorithms::knapsack::c003_a465::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a466")]
                                        // "c003_a466" => Some(tig_algorithms::knapsack::c003_a466::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a467")]
                                        // "c003_a467" => Some(tig_algorithms::knapsack::c003_a467::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a468")]
                                        // "c003_a468" => Some(tig_algorithms::knapsack::c003_a468::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a469")]
                                        // "c003_a469" => Some(tig_algorithms::knapsack::c003_a469::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a470")]
                                        // "c003_a470" => Some(tig_algorithms::knapsack::c003_a470::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a471")]
                                        // "c003_a471" => Some(tig_algorithms::knapsack::c003_a471::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a472")]
                                        // "c003_a472" => Some(tig_algorithms::knapsack::c003_a472::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a473")]
                                        // "c003_a473" => Some(tig_algorithms::knapsack::c003_a473::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a474")]
                                        // "c003_a474" => Some(tig_algorithms::knapsack::c003_a474::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a475")]
                                        // "c003_a475" => Some(tig_algorithms::knapsack::c003_a475::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a476")]
                                        // "c003_a476" => Some(tig_algorithms::knapsack::c003_a476::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a477")]
                                        // "c003_a477" => Some(tig_algorithms::knapsack::c003_a477::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a478")]
                                        // "c003_a478" => Some(tig_algorithms::knapsack::c003_a478::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a479")]
                                        // "c003_a479" => Some(tig_algorithms::knapsack::c003_a479::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a480")]
                                        // "c003_a480" => Some(tig_algorithms::knapsack::c003_a480::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a481")]
                                        // "c003_a481" => Some(tig_algorithms::knapsack::c003_a481::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a482")]
                                        // "c003_a482" => Some(tig_algorithms::knapsack::c003_a482::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a483")]
                                        // "c003_a483" => Some(tig_algorithms::knapsack::c003_a483::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a484")]
                                        // "c003_a484" => Some(tig_algorithms::knapsack::c003_a484::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a485")]
                                        // "c003_a485" => Some(tig_algorithms::knapsack::c003_a485::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a486")]
                                        // "c003_a486" => Some(tig_algorithms::knapsack::c003_a486::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a487")]
                                        // "c003_a487" => Some(tig_algorithms::knapsack::c003_a487::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a488")]
                                        // "c003_a488" => Some(tig_algorithms::knapsack::c003_a488::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a489")]
                                        // "c003_a489" => Some(tig_algorithms::knapsack::c003_a489::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a490")]
                                        // "c003_a490" => Some(tig_algorithms::knapsack::c003_a490::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a491")]
                                        // "c003_a491" => Some(tig_algorithms::knapsack::c003_a491::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a492")]
                                        // "c003_a492" => Some(tig_algorithms::knapsack::c003_a492::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a493")]
                                        // "c003_a493" => Some(tig_algorithms::knapsack::c003_a493::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a494")]
                                        // "c003_a494" => Some(tig_algorithms::knapsack::c003_a494::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a495")]
                                        // "c003_a495" => Some(tig_algorithms::knapsack::c003_a495::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a496")]
                                        // "c003_a496" => Some(tig_algorithms::knapsack::c003_a496::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a497")]
                                        // "c003_a497" => Some(tig_algorithms::knapsack::c003_a497::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a498")]
                                        // "c003_a498" => Some(tig_algorithms::knapsack::c003_a498::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a499")]
                                        // "c003_a499" => Some(tig_algorithms::knapsack::c003_a499::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a500")]
                                        // "c003_a500" => Some(tig_algorithms::knapsack::c003_a500::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a501")]
                                        // "c003_a501" => Some(tig_algorithms::knapsack::c003_a501::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a502")]
                                        // "c003_a502" => Some(tig_algorithms::knapsack::c003_a502::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a503")]
                                        // "c003_a503" => Some(tig_algorithms::knapsack::c003_a503::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a504")]
                                        // "c003_a504" => Some(tig_algorithms::knapsack::c003_a504::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a505")]
                                        // "c003_a505" => Some(tig_algorithms::knapsack::c003_a505::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a506")]
                                        // "c003_a506" => Some(tig_algorithms::knapsack::c003_a506::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a507")]
                                        // "c003_a507" => Some(tig_algorithms::knapsack::c003_a507::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a508")]
                                        // "c003_a508" => Some(tig_algorithms::knapsack::c003_a508::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a509")]
                                        // "c003_a509" => Some(tig_algorithms::knapsack::c003_a509::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a510")]
                                        // "c003_a510" => Some(tig_algorithms::knapsack::c003_a510::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a511")]
                                        // "c003_a511" => Some(tig_algorithms::knapsack::c003_a511::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a512")]
                                        // "c003_a512" => Some(tig_algorithms::knapsack::c003_a512::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a513")]
                                        // "c003_a513" => Some(tig_algorithms::knapsack::c003_a513::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a514")]
                                        // "c003_a514" => Some(tig_algorithms::knapsack::c003_a514::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a515")]
                                        // "c003_a515" => Some(tig_algorithms::knapsack::c003_a515::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a516")]
                                        // "c003_a516" => Some(tig_algorithms::knapsack::c003_a516::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a517")]
                                        // "c003_a517" => Some(tig_algorithms::knapsack::c003_a517::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a518")]
                                        // "c003_a518" => Some(tig_algorithms::knapsack::c003_a518::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a519")]
                                        // "c003_a519" => Some(tig_algorithms::knapsack::c003_a519::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a520")]
                                        // "c003_a520" => Some(tig_algorithms::knapsack::c003_a520::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a521")]
                                        // "c003_a521" => Some(tig_algorithms::knapsack::c003_a521::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a522")]
                                        // "c003_a522" => Some(tig_algorithms::knapsack::c003_a522::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a523")]
                                        // "c003_a523" => Some(tig_algorithms::knapsack::c003_a523::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a524")]
                                        // "c003_a524" => Some(tig_algorithms::knapsack::c003_a524::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a525")]
                                        // "c003_a525" => Some(tig_algorithms::knapsack::c003_a525::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a526")]
                                        // "c003_a526" => Some(tig_algorithms::knapsack::c003_a526::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a527")]
                                        // "c003_a527" => Some(tig_algorithms::knapsack::c003_a527::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a528")]
                                        // "c003_a528" => Some(tig_algorithms::knapsack::c003_a528::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a529")]
                                        // "c003_a529" => Some(tig_algorithms::knapsack::c003_a529::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a530")]
                                        // "c003_a530" => Some(tig_algorithms::knapsack::c003_a530::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a531")]
                                        // "c003_a531" => Some(tig_algorithms::knapsack::c003_a531::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a532")]
                                        // "c003_a532" => Some(tig_algorithms::knapsack::c003_a532::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a533")]
                                        // "c003_a533" => Some(tig_algorithms::knapsack::c003_a533::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a534")]
                                        // "c003_a534" => Some(tig_algorithms::knapsack::c003_a534::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a535")]
                                        // "c003_a535" => Some(tig_algorithms::knapsack::c003_a535::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a536")]
                                        // "c003_a536" => Some(tig_algorithms::knapsack::c003_a536::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a537")]
                                        // "c003_a537" => Some(tig_algorithms::knapsack::c003_a537::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a538")]
                                        // "c003_a538" => Some(tig_algorithms::knapsack::c003_a538::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a539")]
                                        // "c003_a539" => Some(tig_algorithms::knapsack::c003_a539::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a540")]
                                        // "c003_a540" => Some(tig_algorithms::knapsack::c003_a540::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a541")]
                                        // "c003_a541" => Some(tig_algorithms::knapsack::c003_a541::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a542")]
                                        // "c003_a542" => Some(tig_algorithms::knapsack::c003_a542::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a543")]
                                        // "c003_a543" => Some(tig_algorithms::knapsack::c003_a543::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a544")]
                                        // "c003_a544" => Some(tig_algorithms::knapsack::c003_a544::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a545")]
                                        // "c003_a545" => Some(tig_algorithms::knapsack::c003_a545::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a546")]
                                        // "c003_a546" => Some(tig_algorithms::knapsack::c003_a546::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a547")]
                                        // "c003_a547" => Some(tig_algorithms::knapsack::c003_a547::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a548")]
                                        // "c003_a548" => Some(tig_algorithms::knapsack::c003_a548::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a549")]
                                        // "c003_a549" => Some(tig_algorithms::knapsack::c003_a549::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a550")]
                                        // "c003_a550" => Some(tig_algorithms::knapsack::c003_a550::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a551")]
                                        // "c003_a551" => Some(tig_algorithms::knapsack::c003_a551::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a552")]
                                        // "c003_a552" => Some(tig_algorithms::knapsack::c003_a552::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a553")]
                                        // "c003_a553" => Some(tig_algorithms::knapsack::c003_a553::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a554")]
                                        // "c003_a554" => Some(tig_algorithms::knapsack::c003_a554::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a555")]
                                        // "c003_a555" => Some(tig_algorithms::knapsack::c003_a555::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a556")]
                                        // "c003_a556" => Some(tig_algorithms::knapsack::c003_a556::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a557")]
                                        // "c003_a557" => Some(tig_algorithms::knapsack::c003_a557::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a558")]
                                        // "c003_a558" => Some(tig_algorithms::knapsack::c003_a558::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a559")]
                                        // "c003_a559" => Some(tig_algorithms::knapsack::c003_a559::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a560")]
                                        // "c003_a560" => Some(tig_algorithms::knapsack::c003_a560::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a561")]
                                        // "c003_a561" => Some(tig_algorithms::knapsack::c003_a561::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a562")]
                                        // "c003_a562" => Some(tig_algorithms::knapsack::c003_a562::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a563")]
                                        // "c003_a563" => Some(tig_algorithms::knapsack::c003_a563::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a564")]
                                        // "c003_a564" => Some(tig_algorithms::knapsack::c003_a564::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a565")]
                                        // "c003_a565" => Some(tig_algorithms::knapsack::c003_a565::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a566")]
                                        // "c003_a566" => Some(tig_algorithms::knapsack::c003_a566::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a567")]
                                        // "c003_a567" => Some(tig_algorithms::knapsack::c003_a567::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a568")]
                                        // "c003_a568" => Some(tig_algorithms::knapsack::c003_a568::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a569")]
                                        // "c003_a569" => Some(tig_algorithms::knapsack::c003_a569::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a570")]
                                        // "c003_a570" => Some(tig_algorithms::knapsack::c003_a570::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a571")]
                                        // "c003_a571" => Some(tig_algorithms::knapsack::c003_a571::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a572")]
                                        // "c003_a572" => Some(tig_algorithms::knapsack::c003_a572::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a573")]
                                        // "c003_a573" => Some(tig_algorithms::knapsack::c003_a573::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a574")]
                                        // "c003_a574" => Some(tig_algorithms::knapsack::c003_a574::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a575")]
                                        // "c003_a575" => Some(tig_algorithms::knapsack::c003_a575::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a576")]
                                        // "c003_a576" => Some(tig_algorithms::knapsack::c003_a576::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a577")]
                                        // "c003_a577" => Some(tig_algorithms::knapsack::c003_a577::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a578")]
                                        // "c003_a578" => Some(tig_algorithms::knapsack::c003_a578::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a579")]
                                        // "c003_a579" => Some(tig_algorithms::knapsack::c003_a579::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a580")]
                                        // "c003_a580" => Some(tig_algorithms::knapsack::c003_a580::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a581")]
                                        // "c003_a581" => Some(tig_algorithms::knapsack::c003_a581::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a582")]
                                        // "c003_a582" => Some(tig_algorithms::knapsack::c003_a582::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a583")]
                                        // "c003_a583" => Some(tig_algorithms::knapsack::c003_a583::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a584")]
                                        // "c003_a584" => Some(tig_algorithms::knapsack::c003_a584::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a585")]
                                        // "c003_a585" => Some(tig_algorithms::knapsack::c003_a585::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a586")]
                                        // "c003_a586" => Some(tig_algorithms::knapsack::c003_a586::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a587")]
                                        // "c003_a587" => Some(tig_algorithms::knapsack::c003_a587::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a588")]
                                        // "c003_a588" => Some(tig_algorithms::knapsack::c003_a588::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a589")]
                                        // "c003_a589" => Some(tig_algorithms::knapsack::c003_a589::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a590")]
                                        // "c003_a590" => Some(tig_algorithms::knapsack::c003_a590::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a591")]
                                        // "c003_a591" => Some(tig_algorithms::knapsack::c003_a591::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a592")]
                                        // "c003_a592" => Some(tig_algorithms::knapsack::c003_a592::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a593")]
                                        // "c003_a593" => Some(tig_algorithms::knapsack::c003_a593::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a594")]
                                        // "c003_a594" => Some(tig_algorithms::knapsack::c003_a594::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a595")]
                                        // "c003_a595" => Some(tig_algorithms::knapsack::c003_a595::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a596")]
                                        // "c003_a596" => Some(tig_algorithms::knapsack::c003_a596::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a597")]
                                        // "c003_a597" => Some(tig_algorithms::knapsack::c003_a597::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a598")]
                                        // "c003_a598" => Some(tig_algorithms::knapsack::c003_a598::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a599")]
                                        // "c003_a599" => Some(tig_algorithms::knapsack::c003_a599::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a600")]
                                        // "c003_a600" => Some(tig_algorithms::knapsack::c003_a600::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a601")]
                                        // "c003_a601" => Some(tig_algorithms::knapsack::c003_a601::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a602")]
                                        // "c003_a602" => Some(tig_algorithms::knapsack::c003_a602::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a603")]
                                        // "c003_a603" => Some(tig_algorithms::knapsack::c003_a603::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a604")]
                                        // "c003_a604" => Some(tig_algorithms::knapsack::c003_a604::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a605")]
                                        // "c003_a605" => Some(tig_algorithms::knapsack::c003_a605::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a606")]
                                        // "c003_a606" => Some(tig_algorithms::knapsack::c003_a606::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a607")]
                                        // "c003_a607" => Some(tig_algorithms::knapsack::c003_a607::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a608")]
                                        // "c003_a608" => Some(tig_algorithms::knapsack::c003_a608::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a609")]
                                        // "c003_a609" => Some(tig_algorithms::knapsack::c003_a609::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a610")]
                                        // "c003_a610" => Some(tig_algorithms::knapsack::c003_a610::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a611")]
                                        // "c003_a611" => Some(tig_algorithms::knapsack::c003_a611::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a612")]
                                        // "c003_a612" => Some(tig_algorithms::knapsack::c003_a612::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a613")]
                                        // "c003_a613" => Some(tig_algorithms::knapsack::c003_a613::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a614")]
                                        // "c003_a614" => Some(tig_algorithms::knapsack::c003_a614::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a615")]
                                        // "c003_a615" => Some(tig_algorithms::knapsack::c003_a615::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a616")]
                                        // "c003_a616" => Some(tig_algorithms::knapsack::c003_a616::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a617")]
                                        // "c003_a617" => Some(tig_algorithms::knapsack::c003_a617::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a618")]
                                        // "c003_a618" => Some(tig_algorithms::knapsack::c003_a618::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a619")]
                                        // "c003_a619" => Some(tig_algorithms::knapsack::c003_a619::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a620")]
                                        // "c003_a620" => Some(tig_algorithms::knapsack::c003_a620::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a621")]
                                        // "c003_a621" => Some(tig_algorithms::knapsack::c003_a621::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a622")]
                                        // "c003_a622" => Some(tig_algorithms::knapsack::c003_a622::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a623")]
                                        // "c003_a623" => Some(tig_algorithms::knapsack::c003_a623::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a624")]
                                        // "c003_a624" => Some(tig_algorithms::knapsack::c003_a624::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a625")]
                                        // "c003_a625" => Some(tig_algorithms::knapsack::c003_a625::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a626")]
                                        // "c003_a626" => Some(tig_algorithms::knapsack::c003_a626::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a627")]
                                        // "c003_a627" => Some(tig_algorithms::knapsack::c003_a627::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a628")]
                                        // "c003_a628" => Some(tig_algorithms::knapsack::c003_a628::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a629")]
                                        // "c003_a629" => Some(tig_algorithms::knapsack::c003_a629::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a630")]
                                        // "c003_a630" => Some(tig_algorithms::knapsack::c003_a630::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a631")]
                                        // "c003_a631" => Some(tig_algorithms::knapsack::c003_a631::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a632")]
                                        // "c003_a632" => Some(tig_algorithms::knapsack::c003_a632::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a633")]
                                        // "c003_a633" => Some(tig_algorithms::knapsack::c003_a633::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a634")]
                                        // "c003_a634" => Some(tig_algorithms::knapsack::c003_a634::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a635")]
                                        // "c003_a635" => Some(tig_algorithms::knapsack::c003_a635::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a636")]
                                        // "c003_a636" => Some(tig_algorithms::knapsack::c003_a636::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a637")]
                                        // "c003_a637" => Some(tig_algorithms::knapsack::c003_a637::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a638")]
                                        // "c003_a638" => Some(tig_algorithms::knapsack::c003_a638::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a639")]
                                        // "c003_a639" => Some(tig_algorithms::knapsack::c003_a639::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a640")]
                                        // "c003_a640" => Some(tig_algorithms::knapsack::c003_a640::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a641")]
                                        // "c003_a641" => Some(tig_algorithms::knapsack::c003_a641::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a642")]
                                        // "c003_a642" => Some(tig_algorithms::knapsack::c003_a642::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a643")]
                                        // "c003_a643" => Some(tig_algorithms::knapsack::c003_a643::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a644")]
                                        // "c003_a644" => Some(tig_algorithms::knapsack::c003_a644::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a645")]
                                        // "c003_a645" => Some(tig_algorithms::knapsack::c003_a645::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a646")]
                                        // "c003_a646" => Some(tig_algorithms::knapsack::c003_a646::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a647")]
                                        // "c003_a647" => Some(tig_algorithms::knapsack::c003_a647::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a648")]
                                        // "c003_a648" => Some(tig_algorithms::knapsack::c003_a648::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a649")]
                                        // "c003_a649" => Some(tig_algorithms::knapsack::c003_a649::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a650")]
                                        // "c003_a650" => Some(tig_algorithms::knapsack::c003_a650::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a651")]
                                        // "c003_a651" => Some(tig_algorithms::knapsack::c003_a651::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a652")]
                                        // "c003_a652" => Some(tig_algorithms::knapsack::c003_a652::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a653")]
                                        // "c003_a653" => Some(tig_algorithms::knapsack::c003_a653::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a654")]
                                        // "c003_a654" => Some(tig_algorithms::knapsack::c003_a654::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a655")]
                                        // "c003_a655" => Some(tig_algorithms::knapsack::c003_a655::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a656")]
                                        // "c003_a656" => Some(tig_algorithms::knapsack::c003_a656::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a657")]
                                        // "c003_a657" => Some(tig_algorithms::knapsack::c003_a657::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a658")]
                                        // "c003_a658" => Some(tig_algorithms::knapsack::c003_a658::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a659")]
                                        // "c003_a659" => Some(tig_algorithms::knapsack::c003_a659::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a660")]
                                        // "c003_a660" => Some(tig_algorithms::knapsack::c003_a660::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a661")]
                                        // "c003_a661" => Some(tig_algorithms::knapsack::c003_a661::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a662")]
                                        // "c003_a662" => Some(tig_algorithms::knapsack::c003_a662::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a663")]
                                        // "c003_a663" => Some(tig_algorithms::knapsack::c003_a663::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a664")]
                                        // "c003_a664" => Some(tig_algorithms::knapsack::c003_a664::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a665")]
                                        // "c003_a665" => Some(tig_algorithms::knapsack::c003_a665::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a666")]
                                        // "c003_a666" => Some(tig_algorithms::knapsack::c003_a666::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a667")]
                                        // "c003_a667" => Some(tig_algorithms::knapsack::c003_a667::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a668")]
                                        // "c003_a668" => Some(tig_algorithms::knapsack::c003_a668::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a669")]
                                        // "c003_a669" => Some(tig_algorithms::knapsack::c003_a669::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a670")]
                                        // "c003_a670" => Some(tig_algorithms::knapsack::c003_a670::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a671")]
                                        // "c003_a671" => Some(tig_algorithms::knapsack::c003_a671::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a672")]
                                        // "c003_a672" => Some(tig_algorithms::knapsack::c003_a672::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a673")]
                                        // "c003_a673" => Some(tig_algorithms::knapsack::c003_a673::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a674")]
                                        // "c003_a674" => Some(tig_algorithms::knapsack::c003_a674::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a675")]
                                        // "c003_a675" => Some(tig_algorithms::knapsack::c003_a675::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a676")]
                                        // "c003_a676" => Some(tig_algorithms::knapsack::c003_a676::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a677")]
                                        // "c003_a677" => Some(tig_algorithms::knapsack::c003_a677::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a678")]
                                        // "c003_a678" => Some(tig_algorithms::knapsack::c003_a678::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a679")]
                                        // "c003_a679" => Some(tig_algorithms::knapsack::c003_a679::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a680")]
                                        // "c003_a680" => Some(tig_algorithms::knapsack::c003_a680::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a681")]
                                        // "c003_a681" => Some(tig_algorithms::knapsack::c003_a681::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a682")]
                                        // "c003_a682" => Some(tig_algorithms::knapsack::c003_a682::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a683")]
                                        // "c003_a683" => Some(tig_algorithms::knapsack::c003_a683::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a684")]
                                        // "c003_a684" => Some(tig_algorithms::knapsack::c003_a684::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a685")]
                                        // "c003_a685" => Some(tig_algorithms::knapsack::c003_a685::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a686")]
                                        // "c003_a686" => Some(tig_algorithms::knapsack::c003_a686::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a687")]
                                        // "c003_a687" => Some(tig_algorithms::knapsack::c003_a687::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a688")]
                                        // "c003_a688" => Some(tig_algorithms::knapsack::c003_a688::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a689")]
                                        // "c003_a689" => Some(tig_algorithms::knapsack::c003_a689::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a690")]
                                        // "c003_a690" => Some(tig_algorithms::knapsack::c003_a690::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a691")]
                                        // "c003_a691" => Some(tig_algorithms::knapsack::c003_a691::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a692")]
                                        // "c003_a692" => Some(tig_algorithms::knapsack::c003_a692::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a693")]
                                        // "c003_a693" => Some(tig_algorithms::knapsack::c003_a693::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a694")]
                                        // "c003_a694" => Some(tig_algorithms::knapsack::c003_a694::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a695")]
                                        // "c003_a695" => Some(tig_algorithms::knapsack::c003_a695::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a696")]
                                        // "c003_a696" => Some(tig_algorithms::knapsack::c003_a696::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a697")]
                                        // "c003_a697" => Some(tig_algorithms::knapsack::c003_a697::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a698")]
                                        // "c003_a698" => Some(tig_algorithms::knapsack::c003_a698::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a699")]
                                        // "c003_a699" => Some(tig_algorithms::knapsack::c003_a699::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a700")]
                                        // "c003_a700" => Some(tig_algorithms::knapsack::c003_a700::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a701")]
                                        // "c003_a701" => Some(tig_algorithms::knapsack::c003_a701::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a702")]
                                        // "c003_a702" => Some(tig_algorithms::knapsack::c003_a702::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a703")]
                                        // "c003_a703" => Some(tig_algorithms::knapsack::c003_a703::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a704")]
                                        // "c003_a704" => Some(tig_algorithms::knapsack::c003_a704::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a705")]
                                        // "c003_a705" => Some(tig_algorithms::knapsack::c003_a705::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a706")]
                                        // "c003_a706" => Some(tig_algorithms::knapsack::c003_a706::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a707")]
                                        // "c003_a707" => Some(tig_algorithms::knapsack::c003_a707::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a708")]
                                        // "c003_a708" => Some(tig_algorithms::knapsack::c003_a708::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a709")]
                                        // "c003_a709" => Some(tig_algorithms::knapsack::c003_a709::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a710")]
                                        // "c003_a710" => Some(tig_algorithms::knapsack::c003_a710::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a711")]
                                        // "c003_a711" => Some(tig_algorithms::knapsack::c003_a711::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a712")]
                                        // "c003_a712" => Some(tig_algorithms::knapsack::c003_a712::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a713")]
                                        // "c003_a713" => Some(tig_algorithms::knapsack::c003_a713::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a714")]
                                        // "c003_a714" => Some(tig_algorithms::knapsack::c003_a714::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a715")]
                                        // "c003_a715" => Some(tig_algorithms::knapsack::c003_a715::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a716")]
                                        // "c003_a716" => Some(tig_algorithms::knapsack::c003_a716::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a717")]
                                        // "c003_a717" => Some(tig_algorithms::knapsack::c003_a717::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a718")]
                                        // "c003_a718" => Some(tig_algorithms::knapsack::c003_a718::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a719")]
                                        // "c003_a719" => Some(tig_algorithms::knapsack::c003_a719::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a720")]
                                        // "c003_a720" => Some(tig_algorithms::knapsack::c003_a720::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a721")]
                                        // "c003_a721" => Some(tig_algorithms::knapsack::c003_a721::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a722")]
                                        // "c003_a722" => Some(tig_algorithms::knapsack::c003_a722::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a723")]
                                        // "c003_a723" => Some(tig_algorithms::knapsack::c003_a723::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a724")]
                                        // "c003_a724" => Some(tig_algorithms::knapsack::c003_a724::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a725")]
                                        // "c003_a725" => Some(tig_algorithms::knapsack::c003_a725::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a726")]
                                        // "c003_a726" => Some(tig_algorithms::knapsack::c003_a726::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a727")]
                                        // "c003_a727" => Some(tig_algorithms::knapsack::c003_a727::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a728")]
                                        // "c003_a728" => Some(tig_algorithms::knapsack::c003_a728::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a729")]
                                        // "c003_a729" => Some(tig_algorithms::knapsack::c003_a729::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a730")]
                                        // "c003_a730" => Some(tig_algorithms::knapsack::c003_a730::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a731")]
                                        // "c003_a731" => Some(tig_algorithms::knapsack::c003_a731::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a732")]
                                        // "c003_a732" => Some(tig_algorithms::knapsack::c003_a732::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a733")]
                                        // "c003_a733" => Some(tig_algorithms::knapsack::c003_a733::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a734")]
                                        // "c003_a734" => Some(tig_algorithms::knapsack::c003_a734::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a735")]
                                        // "c003_a735" => Some(tig_algorithms::knapsack::c003_a735::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a736")]
                                        // "c003_a736" => Some(tig_algorithms::knapsack::c003_a736::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a737")]
                                        // "c003_a737" => Some(tig_algorithms::knapsack::c003_a737::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a738")]
                                        // "c003_a738" => Some(tig_algorithms::knapsack::c003_a738::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a739")]
                                        // "c003_a739" => Some(tig_algorithms::knapsack::c003_a739::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a740")]
                                        // "c003_a740" => Some(tig_algorithms::knapsack::c003_a740::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a741")]
                                        // "c003_a741" => Some(tig_algorithms::knapsack::c003_a741::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a742")]
                                        // "c003_a742" => Some(tig_algorithms::knapsack::c003_a742::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a743")]
                                        // "c003_a743" => Some(tig_algorithms::knapsack::c003_a743::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a744")]
                                        // "c003_a744" => Some(tig_algorithms::knapsack::c003_a744::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a745")]
                                        // "c003_a745" => Some(tig_algorithms::knapsack::c003_a745::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a746")]
                                        // "c003_a746" => Some(tig_algorithms::knapsack::c003_a746::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a747")]
                                        // "c003_a747" => Some(tig_algorithms::knapsack::c003_a747::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a748")]
                                        // "c003_a748" => Some(tig_algorithms::knapsack::c003_a748::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a749")]
                                        // "c003_a749" => Some(tig_algorithms::knapsack::c003_a749::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a750")]
                                        // "c003_a750" => Some(tig_algorithms::knapsack::c003_a750::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a751")]
                                        // "c003_a751" => Some(tig_algorithms::knapsack::c003_a751::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a752")]
                                        // "c003_a752" => Some(tig_algorithms::knapsack::c003_a752::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a753")]
                                        // "c003_a753" => Some(tig_algorithms::knapsack::c003_a753::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a754")]
                                        // "c003_a754" => Some(tig_algorithms::knapsack::c003_a754::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a755")]
                                        // "c003_a755" => Some(tig_algorithms::knapsack::c003_a755::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a756")]
                                        // "c003_a756" => Some(tig_algorithms::knapsack::c003_a756::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a757")]
                                        // "c003_a757" => Some(tig_algorithms::knapsack::c003_a757::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a758")]
                                        // "c003_a758" => Some(tig_algorithms::knapsack::c003_a758::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a759")]
                                        // "c003_a759" => Some(tig_algorithms::knapsack::c003_a759::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a760")]
                                        // "c003_a760" => Some(tig_algorithms::knapsack::c003_a760::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a761")]
                                        // "c003_a761" => Some(tig_algorithms::knapsack::c003_a761::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a762")]
                                        // "c003_a762" => Some(tig_algorithms::knapsack::c003_a762::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a763")]
                                        // "c003_a763" => Some(tig_algorithms::knapsack::c003_a763::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a764")]
                                        // "c003_a764" => Some(tig_algorithms::knapsack::c003_a764::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a765")]
                                        // "c003_a765" => Some(tig_algorithms::knapsack::c003_a765::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a766")]
                                        // "c003_a766" => Some(tig_algorithms::knapsack::c003_a766::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a767")]
                                        // "c003_a767" => Some(tig_algorithms::knapsack::c003_a767::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a768")]
                                        // "c003_a768" => Some(tig_algorithms::knapsack::c003_a768::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a769")]
                                        // "c003_a769" => Some(tig_algorithms::knapsack::c003_a769::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a770")]
                                        // "c003_a770" => Some(tig_algorithms::knapsack::c003_a770::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a771")]
                                        // "c003_a771" => Some(tig_algorithms::knapsack::c003_a771::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a772")]
                                        // "c003_a772" => Some(tig_algorithms::knapsack::c003_a772::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a773")]
                                        // "c003_a773" => Some(tig_algorithms::knapsack::c003_a773::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a774")]
                                        // "c003_a774" => Some(tig_algorithms::knapsack::c003_a774::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a775")]
                                        // "c003_a775" => Some(tig_algorithms::knapsack::c003_a775::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a776")]
                                        // "c003_a776" => Some(tig_algorithms::knapsack::c003_a776::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a777")]
                                        // "c003_a777" => Some(tig_algorithms::knapsack::c003_a777::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a778")]
                                        // "c003_a778" => Some(tig_algorithms::knapsack::c003_a778::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a779")]
                                        // "c003_a779" => Some(tig_algorithms::knapsack::c003_a779::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a780")]
                                        // "c003_a780" => Some(tig_algorithms::knapsack::c003_a780::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a781")]
                                        // "c003_a781" => Some(tig_algorithms::knapsack::c003_a781::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a782")]
                                        // "c003_a782" => Some(tig_algorithms::knapsack::c003_a782::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a783")]
                                        // "c003_a783" => Some(tig_algorithms::knapsack::c003_a783::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a784")]
                                        // "c003_a784" => Some(tig_algorithms::knapsack::c003_a784::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a785")]
                                        // "c003_a785" => Some(tig_algorithms::knapsack::c003_a785::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a786")]
                                        // "c003_a786" => Some(tig_algorithms::knapsack::c003_a786::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a787")]
                                        // "c003_a787" => Some(tig_algorithms::knapsack::c003_a787::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a788")]
                                        // "c003_a788" => Some(tig_algorithms::knapsack::c003_a788::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a789")]
                                        // "c003_a789" => Some(tig_algorithms::knapsack::c003_a789::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a790")]
                                        // "c003_a790" => Some(tig_algorithms::knapsack::c003_a790::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a791")]
                                        // "c003_a791" => Some(tig_algorithms::knapsack::c003_a791::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a792")]
                                        // "c003_a792" => Some(tig_algorithms::knapsack::c003_a792::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a793")]
                                        // "c003_a793" => Some(tig_algorithms::knapsack::c003_a793::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a794")]
                                        // "c003_a794" => Some(tig_algorithms::knapsack::c003_a794::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a795")]
                                        // "c003_a795" => Some(tig_algorithms::knapsack::c003_a795::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a796")]
                                        // "c003_a796" => Some(tig_algorithms::knapsack::c003_a796::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a797")]
                                        // "c003_a797" => Some(tig_algorithms::knapsack::c003_a797::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a798")]
                                        // "c003_a798" => Some(tig_algorithms::knapsack::c003_a798::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a799")]
                                        // "c003_a799" => Some(tig_algorithms::knapsack::c003_a799::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a800")]
                                        // "c003_a800" => Some(tig_algorithms::knapsack::c003_a800::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a801")]
                                        // "c003_a801" => Some(tig_algorithms::knapsack::c003_a801::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a802")]
                                        // "c003_a802" => Some(tig_algorithms::knapsack::c003_a802::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a803")]
                                        // "c003_a803" => Some(tig_algorithms::knapsack::c003_a803::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a804")]
                                        // "c003_a804" => Some(tig_algorithms::knapsack::c003_a804::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a805")]
                                        // "c003_a805" => Some(tig_algorithms::knapsack::c003_a805::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a806")]
                                        // "c003_a806" => Some(tig_algorithms::knapsack::c003_a806::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a807")]
                                        // "c003_a807" => Some(tig_algorithms::knapsack::c003_a807::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a808")]
                                        // "c003_a808" => Some(tig_algorithms::knapsack::c003_a808::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a809")]
                                        // "c003_a809" => Some(tig_algorithms::knapsack::c003_a809::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a810")]
                                        // "c003_a810" => Some(tig_algorithms::knapsack::c003_a810::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a811")]
                                        // "c003_a811" => Some(tig_algorithms::knapsack::c003_a811::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a812")]
                                        // "c003_a812" => Some(tig_algorithms::knapsack::c003_a812::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a813")]
                                        // "c003_a813" => Some(tig_algorithms::knapsack::c003_a813::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a814")]
                                        // "c003_a814" => Some(tig_algorithms::knapsack::c003_a814::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a815")]
                                        // "c003_a815" => Some(tig_algorithms::knapsack::c003_a815::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a816")]
                                        // "c003_a816" => Some(tig_algorithms::knapsack::c003_a816::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a817")]
                                        // "c003_a817" => Some(tig_algorithms::knapsack::c003_a817::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a818")]
                                        // "c003_a818" => Some(tig_algorithms::knapsack::c003_a818::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a819")]
                                        // "c003_a819" => Some(tig_algorithms::knapsack::c003_a819::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a820")]
                                        // "c003_a820" => Some(tig_algorithms::knapsack::c003_a820::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a821")]
                                        // "c003_a821" => Some(tig_algorithms::knapsack::c003_a821::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a822")]
                                        // "c003_a822" => Some(tig_algorithms::knapsack::c003_a822::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a823")]
                                        // "c003_a823" => Some(tig_algorithms::knapsack::c003_a823::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a824")]
                                        // "c003_a824" => Some(tig_algorithms::knapsack::c003_a824::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a825")]
                                        // "c003_a825" => Some(tig_algorithms::knapsack::c003_a825::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a826")]
                                        // "c003_a826" => Some(tig_algorithms::knapsack::c003_a826::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a827")]
                                        // "c003_a827" => Some(tig_algorithms::knapsack::c003_a827::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a828")]
                                        // "c003_a828" => Some(tig_algorithms::knapsack::c003_a828::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a829")]
                                        // "c003_a829" => Some(tig_algorithms::knapsack::c003_a829::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a830")]
                                        // "c003_a830" => Some(tig_algorithms::knapsack::c003_a830::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a831")]
                                        // "c003_a831" => Some(tig_algorithms::knapsack::c003_a831::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a832")]
                                        // "c003_a832" => Some(tig_algorithms::knapsack::c003_a832::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a833")]
                                        // "c003_a833" => Some(tig_algorithms::knapsack::c003_a833::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a834")]
                                        // "c003_a834" => Some(tig_algorithms::knapsack::c003_a834::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a835")]
                                        // "c003_a835" => Some(tig_algorithms::knapsack::c003_a835::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a836")]
                                        // "c003_a836" => Some(tig_algorithms::knapsack::c003_a836::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a837")]
                                        // "c003_a837" => Some(tig_algorithms::knapsack::c003_a837::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a838")]
                                        // "c003_a838" => Some(tig_algorithms::knapsack::c003_a838::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a839")]
                                        // "c003_a839" => Some(tig_algorithms::knapsack::c003_a839::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a840")]
                                        // "c003_a840" => Some(tig_algorithms::knapsack::c003_a840::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a841")]
                                        // "c003_a841" => Some(tig_algorithms::knapsack::c003_a841::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a842")]
                                        // "c003_a842" => Some(tig_algorithms::knapsack::c003_a842::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a843")]
                                        // "c003_a843" => Some(tig_algorithms::knapsack::c003_a843::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a844")]
                                        // "c003_a844" => Some(tig_algorithms::knapsack::c003_a844::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a845")]
                                        // "c003_a845" => Some(tig_algorithms::knapsack::c003_a845::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a846")]
                                        // "c003_a846" => Some(tig_algorithms::knapsack::c003_a846::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a847")]
                                        // "c003_a847" => Some(tig_algorithms::knapsack::c003_a847::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a848")]
                                        // "c003_a848" => Some(tig_algorithms::knapsack::c003_a848::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a849")]
                                        // "c003_a849" => Some(tig_algorithms::knapsack::c003_a849::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a850")]
                                        // "c003_a850" => Some(tig_algorithms::knapsack::c003_a850::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a851")]
                                        // "c003_a851" => Some(tig_algorithms::knapsack::c003_a851::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a852")]
                                        // "c003_a852" => Some(tig_algorithms::knapsack::c003_a852::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a853")]
                                        // "c003_a853" => Some(tig_algorithms::knapsack::c003_a853::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a854")]
                                        // "c003_a854" => Some(tig_algorithms::knapsack::c003_a854::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a855")]
                                        // "c003_a855" => Some(tig_algorithms::knapsack::c003_a855::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a856")]
                                        // "c003_a856" => Some(tig_algorithms::knapsack::c003_a856::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a857")]
                                        // "c003_a857" => Some(tig_algorithms::knapsack::c003_a857::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a858")]
                                        // "c003_a858" => Some(tig_algorithms::knapsack::c003_a858::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a859")]
                                        // "c003_a859" => Some(tig_algorithms::knapsack::c003_a859::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a860")]
                                        // "c003_a860" => Some(tig_algorithms::knapsack::c003_a860::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a861")]
                                        // "c003_a861" => Some(tig_algorithms::knapsack::c003_a861::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a862")]
                                        // "c003_a862" => Some(tig_algorithms::knapsack::c003_a862::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a863")]
                                        // "c003_a863" => Some(tig_algorithms::knapsack::c003_a863::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a864")]
                                        // "c003_a864" => Some(tig_algorithms::knapsack::c003_a864::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a865")]
                                        // "c003_a865" => Some(tig_algorithms::knapsack::c003_a865::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a866")]
                                        // "c003_a866" => Some(tig_algorithms::knapsack::c003_a866::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a867")]
                                        // "c003_a867" => Some(tig_algorithms::knapsack::c003_a867::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a868")]
                                        // "c003_a868" => Some(tig_algorithms::knapsack::c003_a868::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a869")]
                                        // "c003_a869" => Some(tig_algorithms::knapsack::c003_a869::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a870")]
                                        // "c003_a870" => Some(tig_algorithms::knapsack::c003_a870::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a871")]
                                        // "c003_a871" => Some(tig_algorithms::knapsack::c003_a871::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a872")]
                                        // "c003_a872" => Some(tig_algorithms::knapsack::c003_a872::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a873")]
                                        // "c003_a873" => Some(tig_algorithms::knapsack::c003_a873::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a874")]
                                        // "c003_a874" => Some(tig_algorithms::knapsack::c003_a874::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a875")]
                                        // "c003_a875" => Some(tig_algorithms::knapsack::c003_a875::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a876")]
                                        // "c003_a876" => Some(tig_algorithms::knapsack::c003_a876::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a877")]
                                        // "c003_a877" => Some(tig_algorithms::knapsack::c003_a877::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a878")]
                                        // "c003_a878" => Some(tig_algorithms::knapsack::c003_a878::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a879")]
                                        // "c003_a879" => Some(tig_algorithms::knapsack::c003_a879::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a880")]
                                        // "c003_a880" => Some(tig_algorithms::knapsack::c003_a880::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a881")]
                                        // "c003_a881" => Some(tig_algorithms::knapsack::c003_a881::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a882")]
                                        // "c003_a882" => Some(tig_algorithms::knapsack::c003_a882::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a883")]
                                        // "c003_a883" => Some(tig_algorithms::knapsack::c003_a883::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a884")]
                                        // "c003_a884" => Some(tig_algorithms::knapsack::c003_a884::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a885")]
                                        // "c003_a885" => Some(tig_algorithms::knapsack::c003_a885::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a886")]
                                        // "c003_a886" => Some(tig_algorithms::knapsack::c003_a886::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a887")]
                                        // "c003_a887" => Some(tig_algorithms::knapsack::c003_a887::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a888")]
                                        // "c003_a888" => Some(tig_algorithms::knapsack::c003_a888::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a889")]
                                        // "c003_a889" => Some(tig_algorithms::knapsack::c003_a889::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a890")]
                                        // "c003_a890" => Some(tig_algorithms::knapsack::c003_a890::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a891")]
                                        // "c003_a891" => Some(tig_algorithms::knapsack::c003_a891::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a892")]
                                        // "c003_a892" => Some(tig_algorithms::knapsack::c003_a892::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a893")]
                                        // "c003_a893" => Some(tig_algorithms::knapsack::c003_a893::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a894")]
                                        // "c003_a894" => Some(tig_algorithms::knapsack::c003_a894::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a895")]
                                        // "c003_a895" => Some(tig_algorithms::knapsack::c003_a895::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a896")]
                                        // "c003_a896" => Some(tig_algorithms::knapsack::c003_a896::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a897")]
                                        // "c003_a897" => Some(tig_algorithms::knapsack::c003_a897::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a898")]
                                        // "c003_a898" => Some(tig_algorithms::knapsack::c003_a898::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a899")]
                                        // "c003_a899" => Some(tig_algorithms::knapsack::c003_a899::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a900")]
                                        // "c003_a900" => Some(tig_algorithms::knapsack::c003_a900::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a901")]
                                        // "c003_a901" => Some(tig_algorithms::knapsack::c003_a901::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a902")]
                                        // "c003_a902" => Some(tig_algorithms::knapsack::c003_a902::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a903")]
                                        // "c003_a903" => Some(tig_algorithms::knapsack::c003_a903::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a904")]
                                        // "c003_a904" => Some(tig_algorithms::knapsack::c003_a904::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a905")]
                                        // "c003_a905" => Some(tig_algorithms::knapsack::c003_a905::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a906")]
                                        // "c003_a906" => Some(tig_algorithms::knapsack::c003_a906::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a907")]
                                        // "c003_a907" => Some(tig_algorithms::knapsack::c003_a907::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a908")]
                                        // "c003_a908" => Some(tig_algorithms::knapsack::c003_a908::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a909")]
                                        // "c003_a909" => Some(tig_algorithms::knapsack::c003_a909::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a910")]
                                        // "c003_a910" => Some(tig_algorithms::knapsack::c003_a910::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a911")]
                                        // "c003_a911" => Some(tig_algorithms::knapsack::c003_a911::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a912")]
                                        // "c003_a912" => Some(tig_algorithms::knapsack::c003_a912::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a913")]
                                        // "c003_a913" => Some(tig_algorithms::knapsack::c003_a913::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a914")]
                                        // "c003_a914" => Some(tig_algorithms::knapsack::c003_a914::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a915")]
                                        // "c003_a915" => Some(tig_algorithms::knapsack::c003_a915::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a916")]
                                        // "c003_a916" => Some(tig_algorithms::knapsack::c003_a916::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a917")]
                                        // "c003_a917" => Some(tig_algorithms::knapsack::c003_a917::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a918")]
                                        // "c003_a918" => Some(tig_algorithms::knapsack::c003_a918::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a919")]
                                        // "c003_a919" => Some(tig_algorithms::knapsack::c003_a919::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a920")]
                                        // "c003_a920" => Some(tig_algorithms::knapsack::c003_a920::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a921")]
                                        // "c003_a921" => Some(tig_algorithms::knapsack::c003_a921::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a922")]
                                        // "c003_a922" => Some(tig_algorithms::knapsack::c003_a922::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a923")]
                                        // "c003_a923" => Some(tig_algorithms::knapsack::c003_a923::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a924")]
                                        // "c003_a924" => Some(tig_algorithms::knapsack::c003_a924::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a925")]
                                        // "c003_a925" => Some(tig_algorithms::knapsack::c003_a925::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a926")]
                                        // "c003_a926" => Some(tig_algorithms::knapsack::c003_a926::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a927")]
                                        // "c003_a927" => Some(tig_algorithms::knapsack::c003_a927::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a928")]
                                        // "c003_a928" => Some(tig_algorithms::knapsack::c003_a928::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a929")]
                                        // "c003_a929" => Some(tig_algorithms::knapsack::c003_a929::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a930")]
                                        // "c003_a930" => Some(tig_algorithms::knapsack::c003_a930::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a931")]
                                        // "c003_a931" => Some(tig_algorithms::knapsack::c003_a931::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a932")]
                                        // "c003_a932" => Some(tig_algorithms::knapsack::c003_a932::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a933")]
                                        // "c003_a933" => Some(tig_algorithms::knapsack::c003_a933::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a934")]
                                        // "c003_a934" => Some(tig_algorithms::knapsack::c003_a934::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a935")]
                                        // "c003_a935" => Some(tig_algorithms::knapsack::c003_a935::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a936")]
                                        // "c003_a936" => Some(tig_algorithms::knapsack::c003_a936::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a937")]
                                        // "c003_a937" => Some(tig_algorithms::knapsack::c003_a937::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a938")]
                                        // "c003_a938" => Some(tig_algorithms::knapsack::c003_a938::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a939")]
                                        // "c003_a939" => Some(tig_algorithms::knapsack::c003_a939::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a940")]
                                        // "c003_a940" => Some(tig_algorithms::knapsack::c003_a940::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a941")]
                                        // "c003_a941" => Some(tig_algorithms::knapsack::c003_a941::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a942")]
                                        // "c003_a942" => Some(tig_algorithms::knapsack::c003_a942::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a943")]
                                        // "c003_a943" => Some(tig_algorithms::knapsack::c003_a943::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a944")]
                                        // "c003_a944" => Some(tig_algorithms::knapsack::c003_a944::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a945")]
                                        // "c003_a945" => Some(tig_algorithms::knapsack::c003_a945::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a946")]
                                        // "c003_a946" => Some(tig_algorithms::knapsack::c003_a946::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a947")]
                                        // "c003_a947" => Some(tig_algorithms::knapsack::c003_a947::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a948")]
                                        // "c003_a948" => Some(tig_algorithms::knapsack::c003_a948::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a949")]
                                        // "c003_a949" => Some(tig_algorithms::knapsack::c003_a949::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a950")]
                                        // "c003_a950" => Some(tig_algorithms::knapsack::c003_a950::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a951")]
                                        // "c003_a951" => Some(tig_algorithms::knapsack::c003_a951::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a952")]
                                        // "c003_a952" => Some(tig_algorithms::knapsack::c003_a952::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a953")]
                                        // "c003_a953" => Some(tig_algorithms::knapsack::c003_a953::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a954")]
                                        // "c003_a954" => Some(tig_algorithms::knapsack::c003_a954::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a955")]
                                        // "c003_a955" => Some(tig_algorithms::knapsack::c003_a955::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a956")]
                                        // "c003_a956" => Some(tig_algorithms::knapsack::c003_a956::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a957")]
                                        // "c003_a957" => Some(tig_algorithms::knapsack::c003_a957::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a958")]
                                        // "c003_a958" => Some(tig_algorithms::knapsack::c003_a958::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a959")]
                                        // "c003_a959" => Some(tig_algorithms::knapsack::c003_a959::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a960")]
                                        // "c003_a960" => Some(tig_algorithms::knapsack::c003_a960::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a961")]
                                        // "c003_a961" => Some(tig_algorithms::knapsack::c003_a961::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a962")]
                                        // "c003_a962" => Some(tig_algorithms::knapsack::c003_a962::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a963")]
                                        // "c003_a963" => Some(tig_algorithms::knapsack::c003_a963::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a964")]
                                        // "c003_a964" => Some(tig_algorithms::knapsack::c003_a964::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a965")]
                                        // "c003_a965" => Some(tig_algorithms::knapsack::c003_a965::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a966")]
                                        // "c003_a966" => Some(tig_algorithms::knapsack::c003_a966::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a967")]
                                        // "c003_a967" => Some(tig_algorithms::knapsack::c003_a967::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a968")]
                                        // "c003_a968" => Some(tig_algorithms::knapsack::c003_a968::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a969")]
                                        // "c003_a969" => Some(tig_algorithms::knapsack::c003_a969::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a970")]
                                        // "c003_a970" => Some(tig_algorithms::knapsack::c003_a970::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a971")]
                                        // "c003_a971" => Some(tig_algorithms::knapsack::c003_a971::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a972")]
                                        // "c003_a972" => Some(tig_algorithms::knapsack::c003_a972::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a973")]
                                        // "c003_a973" => Some(tig_algorithms::knapsack::c003_a973::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a974")]
                                        // "c003_a974" => Some(tig_algorithms::knapsack::c003_a974::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a975")]
                                        // "c003_a975" => Some(tig_algorithms::knapsack::c003_a975::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a976")]
                                        // "c003_a976" => Some(tig_algorithms::knapsack::c003_a976::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a977")]
                                        // "c003_a977" => Some(tig_algorithms::knapsack::c003_a977::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a978")]
                                        // "c003_a978" => Some(tig_algorithms::knapsack::c003_a978::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a979")]
                                        // "c003_a979" => Some(tig_algorithms::knapsack::c003_a979::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a980")]
                                        // "c003_a980" => Some(tig_algorithms::knapsack::c003_a980::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a981")]
                                        // "c003_a981" => Some(tig_algorithms::knapsack::c003_a981::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a982")]
                                        // "c003_a982" => Some(tig_algorithms::knapsack::c003_a982::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a983")]
                                        // "c003_a983" => Some(tig_algorithms::knapsack::c003_a983::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a984")]
                                        // "c003_a984" => Some(tig_algorithms::knapsack::c003_a984::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a985")]
                                        // "c003_a985" => Some(tig_algorithms::knapsack::c003_a985::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a986")]
                                        // "c003_a986" => Some(tig_algorithms::knapsack::c003_a986::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a987")]
                                        // "c003_a987" => Some(tig_algorithms::knapsack::c003_a987::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a988")]
                                        // "c003_a988" => Some(tig_algorithms::knapsack::c003_a988::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a989")]
                                        // "c003_a989" => Some(tig_algorithms::knapsack::c003_a989::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a990")]
                                        // "c003_a990" => Some(tig_algorithms::knapsack::c003_a990::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a991")]
                                        // "c003_a991" => Some(tig_algorithms::knapsack::c003_a991::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a992")]
                                        // "c003_a992" => Some(tig_algorithms::knapsack::c003_a992::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a993")]
                                        // "c003_a993" => Some(tig_algorithms::knapsack::c003_a993::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a994")]
                                        // "c003_a994" => Some(tig_algorithms::knapsack::c003_a994::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a995")]
                                        // "c003_a995" => Some(tig_algorithms::knapsack::c003_a995::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a996")]
                                        // "c003_a996" => Some(tig_algorithms::knapsack::c003_a996::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a997")]
                                        // "c003_a997" => Some(tig_algorithms::knapsack::c003_a997::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a998")]
                                        // "c003_a998" => Some(tig_algorithms::knapsack::c003_a998::solve_challenge as SolveChallengeFn),
                                        
                                        // #[cfg(feature = "c003_a999")]
                                        // "c003_a999" => Some(tig_algorithms::knapsack::c003_a999::solve_challenge as SolveChallengeFn),

                                        _ => Option::<SolveChallengeFn>::None
                                    }
                                {
                                    let seed = job.settings.calc_seed(nonce);
                                    let challenge = tig_challenges::knapsack::Challenge::generate_instance_from_vec(seed, &job.settings.difficulty).unwrap();
                                    run_wasm = false;
                                    if let Ok(Some(solution)) = solve_challenge(&challenge) {
                                        run_wasm =
                                            challenge.verify_solution(&solution).is_ok();
                                    }
                                }
                            }
                            _ => panic!("Unknown challenge id: {}", job.settings.challenge_id),
                        }
                        if !run_wasm {
                            continue;
                        }
                        if let Ok(ComputeResult::ValidSolution(solution_data)) = compute_solution(
                            &job.settings,
                            nonce,
                            wasm.as_slice(),
                            job.wasm_vm_config.max_memory,
                            job.wasm_vm_config.max_fuel,
                        ) {
                            {
                                let mut solutions_count = (*solutions_count).lock().await;
                                *solutions_count += 1;
                            }
                            if solution_data.calc_solution_signature()
                                <= job.solution_signature_threshold
                            {
                                let mut solutions_data = (*solutions_data).lock().await;
                                (*solutions_data).push(solution_data);
                            }
                        }
                    }
                }
            }
        });
    }
}
