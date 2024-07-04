use neon::prelude::*;

fn strrev(mut cx: FunctionContext) -> JsResult<JsString>{
    let input = cx.argument::<JsString>(0)?.value(&mut cx);
    let reversed = input.chars().rev().collect::<String>();
    Ok(cx.string(reversed))
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("strrev", strrev)?;
    Ok(())
}
