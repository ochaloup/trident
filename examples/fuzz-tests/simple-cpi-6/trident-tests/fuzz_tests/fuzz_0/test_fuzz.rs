use fuzz_instructions::InitializeCaller;
use trident_client::fuzzing::*;
mod fuzz_instructions;
use callee::entry as entry_callee;
use callee::ID as PROGRAM_ID_CALLEE;
use caller::entry as entry_caller;
use caller::ID as PROGRAM_ID_CALLER;
use fuzz_instructions::FuzzInstruction;
const PROGRAM_NAME_CALLEE: &str = "callee";
const PROGRAM_NAME_CALLER: &str = "caller";
struct MyFuzzData;
/// Define instruction sequences for invocation.
/// `pre_ixs` runs at the start, `ixs` in the middle, and `post_ixs` at the end.
/// For example, to call `InitializeFn` at the start of each fuzzing iteration:
/// ```
/// fn pre_ixs(u: &mut arbitrary::Unstructured) ->
/// arbitrary::Result<Vec<FuzzInstruction>> {
///     let init = FuzzInstruction::InitializeFn(InitializeFn::arbitrary(u)?);
///     Ok(vec![init])
/// }
/// ```
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/instructions-sequences/#instructions-sequences
impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_caller = FuzzInstruction::InitializeCaller(InitializeCaller::arbitrary(u)?);
        Ok(vec![init_caller])
    }
    fn ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        Ok(vec![])
    }
    fn post_ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        Ok(vec![])
    }
}
/// `fn fuzz_iteration` runs during every fuzzing iteration.
/// Modification is not required.
fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
) {
    let fuzzing_program_callee = FuzzingProgram::new(
        PROGRAM_NAME_CALLEE,
        &PROGRAM_ID_CALLEE,
        processor!(convert_entry!(entry_callee)),
    );
    let fuzzing_program_caller = FuzzingProgram::new(
        PROGRAM_NAME_CALLER,
        &PROGRAM_ID_CALLER,
        processor!(convert_entry!(entry_caller)),
    );
    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_callee, fuzzing_program_caller], config)
            .unwrap();
    let _ = fuzz_data.run_with_runtime(&mut client, config);
}
fn main() {
    let config = Config::new();
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : MyFuzzData | { fuzz_iteration (fuzz_data , & config) ; });
}
