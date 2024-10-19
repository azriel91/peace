# Aesthetics: Errors

<div style="font-size: 1.5em;">

<pre class="terminal" style="word-break: break-all; text-wrap: wrap;">
thread 'main' panicked at C:\Users\azrielh\work\github\azriel91\peace\examples\envman\src\items\peace_aws_s3_object\s3_object_apply_fns.rs:204:30:
called `Result::unwrap()` on an `Err` value: S3ObjectUploadError { bucket_name: "azrielh-peace-envman-demo-1", object_key: "web_app.tar", aws_desc: "timed out", aws_desc_span: SourceSpan { offset: SourceOffset(0), length: 9 }, error: DispatchFailure(DispatchFailure { source: ConnectorError { kind: Timeout, source: hyper::Error(Connect, HttpTimeoutError { kind: "HTTP connect", duration: 3.1s }), connection: Unknown } }) }
stack backtrace:
   0: &lt;unknown&gt;
   ..
  73: &lt;unknown&gt;
  74: BaseThreadInitThunk
  75: RtlUserThreadStart
</pre>


<pre class="terminal">
<span style='color:#5fffaf'>❯</span> <span style='color:#5fcfdf'>./envman</span> <span style='color:#5fffaf'>clean</span>
..
✅ 5. <span style='color:#5fafff'>s3_bucket</span>        <span style='color:#00af5f'>▰▰▰▰▰▰▰▰▰▰▰▰▰▰</span> done!
❌ 6. <span style='color:#5fafff'>s3_object</span>        <span style='color:#d70000'>▱▱▱▱▱▱▱▱▱▱▱▱▱▱</span> 0/1
                        Failed to upload S3 object: `web_app.tar`.
# <b>Errors</b>
<span style='color:#5fafff'>`s3_object`</span>:
  <span style='color:#f44'>×</span> Failed to upload S3 object: `web_app.tar`.
   ╭────
 <span style='opacity:0.67'>1</span> │ timed out
   · <span style='color:#d9d'><b>─────────</b></span>
   ╰────
  <span style='color:#5fafaf'>help:</span> Make sure you are connected to the internet and try again.
</pre>

</div>
