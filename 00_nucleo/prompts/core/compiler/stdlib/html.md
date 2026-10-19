# L0 — public html module and feature gate
Hash do Código: 60d65ea5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:a933254df6e90a505910759d443fe8b06f2646e2d4a9949bc179326fc9c5c329

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/html.rs`

## Evidence before specification

This façade preserves its public interface, tests and direct reexports. Child owners under `00_nucleo/prompts/core/compiler/stdlib/html/` exclusively own namespace, frame, typed tags/types, global/specific attributes, casts, construction and elem. The clauses below are composition boundaries. Historical source at a51e02804 library.rs:369, html/mod.rs:34–41 and typed.rs:31–114, plus assets pin `94dcb990edb36311ef85162baa26e4a8205e0bf0`, define 112 tags, 76 globals, void/raw classes and typed signatures. Feature and target are independent; construction in a paged target was measured. Initial missing-tag/frame and Module-root stages are superseded by the approved surfaces below. Historical probes at `9f2b6e8715237cb086899a42873660fe744622e8` do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern; internal structure is mechanics, and observed behavior alone is not intent.

Human gates dated 2026-08-25 cover generic elem, the first twelve typed tags, tri-state body, ol/li/a/br and subsequent approved normal/document/ruby/residual groups. The archived original retains exact source, probes, DOM and whitespace observations. Anchor casts were measured from the binary when assets source was unavailable: the lockfile establishes a pin, not invented source lines. A conflicting later source table would refute that characterization rather than erase observed values. Forty-three residual typed routes and frame were explicitly confirmed on 2026-09-16; frame's rendered SVG island differs from literal elem("frame"). Historical negative tests do not deny subsequently approved siblings.

The root-function correction was explicitly confirmed on 2026-09-21 after causal commit `2ae054ab5`: type function, repr html and manual-construction rejection, with the same 112 typed functions plus elem/frame. Five independent language RED cases corrected stale Module assumptions. Historical results (36 HTML/core6059/shell83/infra995 and no new Clippy warnings) certify only that snapshot. cfg-test-ci-2026-10-13/before/refinement-before records duplicated parent/inner cfg in two test modules; removing the inner cfg fails architecture V2.

`lab/results/construction/diagnostics/native-format-catalogue-2026-10-17/` before.json.gz preserves dirty HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/state/hashes. baseline-reuse.json proves the same sources/config/binary as the prior 3956-case, eight-profile matrix. before-batch.json.gz measures 144 detailed cases; before-compat.json.gz measures 456 name/alias/namespace controls with arguments/binaries/outputs. Fifteen where/kind routes differ; one shared native_format pointer cannot identify five schemas. pinned-schema.json.gz preserves nine upstream declarations/blobs/lines/attributes. FuncRepr is pub(crate), refuting the earlier public claim; public-contract-before.json fixes signatures/reexports/structs. This supports internal representation/parity work, not inferred render intent.

## Exclusive current façade and target boundary

make_html_module returns Value::Func with the original native_html_root/namespace; the namespace owner uses internal Format(Html, original NativeFunc), never Value::Module. Preserve repr html, type function and exact manual-call error `cannot be constructed manually`. The elem/frame/112 typed functions and format.html clones preserve all 114 identities, gates, names, casts and attribute order. With FeatureHtml off, retain the gated definition but reject public access with its measured feature diagnostic/hints, not unknown-variable. Target HTML never enables the feature. Feature on allows construction in paged or HTML targets; target-specific consumption belongs to the semantic exporter or frame-island owner.

Generic html.elem requires a positional Str tag with a valid HTML name, optional attrs dictionary of valid names/string values, and optional positional Content body. Preserve attrs/body Unset versus explicit states and measured repr. Add no tag-specific parent/content-model/browser behavior. Typed normal tags accept at most one Content body: omission is HtmlBody::None and supply preserves Content. Void tags reject every positional/body and keep Unset. Raw script/style accept optional Str, never evaluated Content. Named body, excess, invalid types, unknown/data-* attributes or another tag's attributes fail with measured diagnostics/spans. Title takes Content despite its escapable-raw class; it has no special raw-String cast. Omitted attributes are not serialized and there is no permissive string fallback.

All typed tags accept exactly the following 76 globals, with ordered casts/omissions and no duplicates of this data in exporter:

- Str: aria-activedescendant,aria-details,aria-errormessage,aria-keyshortcuts,aria-label,aria-placeholder,aria-roledescription,aria-valuetext,id,is,itemid,nonce,slot,style,title.
- Int: aria-colcount,aria-colindex,aria-colspan,aria-level,aria-posinset,aria-rowcount,aria-rowindex,aria-rowspan,aria-setsize,tabindex. Float: aria-valuemax,aria-valuemin,aria-valuenow.
- Bool serialized true|false: aria-atomic,aria-busy,aria-disabled,aria-modal,aria-multiline,aria-multiselectable,aria-readonly,aria-required,draggable,spellcheck,writingsuggestions. Presence(true→empty,false→omit): autofocus,inert,itemscope.
- Space lists with scalar shorthand: accesskey:char; aria-controls,aria-describedby,aria-flowto,aria-labelledby,aria-owns,class,itemprop,itemref,itemtype:Str. aria-relevant tokens additions|additions text|all|removals|text.
- aria-checked/pressed Bool|mixed; aria-expanded/hidden/selected Bool|None(undefined); aria-autocomplete none|inline|list|both; aria-current Bool|page|step|location|date|time; aria-haspopup Bool|menu|listbox|tree|grid|dialog; aria-invalid Bool|grammar|spelling; aria-live assertive|off|polite; aria-orientation horizontal|undefined|vertical; aria-sort none|ascending|descending|other.
- autocapitalize on|off|none|sentences|words|characters; autocorrect on|off; contenteditable Bool|plaintext-only; dir ltr|rtl|auto; enterkeyhint enter|done|go|next|previous|search|send; hidden Presence|until-found; inputmode none|text|tel|email|url|numeric|decimal|search; lang Str|None(empty string); popover auto|manual; role None or exactly 80ARIA tokens in pinned ATTR_STRINGS[237..317]; translate Bool→yes|no.

Normal global-only tag catalog: div,span,p,h1,h2,h3,h4,h5,h6,strong,em,ul; abbr,address,article,aside,b,bdi,bdo,cite,code,dfn,i,kbd; dd,dl,dt,figcaption,figure,footer,header,hgroup,legend,main,mark,menu; nav,picture,pre,s,samp,search,section,small,sub,sup,u,var; datalist,noscript,summary; ruby,rp,rt; html,head,body,title. They have no specific attributes/parent/context/target validation. HtmlPre body is ordinary normalized Content, not raw; Picture accepts empty/text. Document hierarchy/uniqueness/display/phrasing/ruby boundaries and whitespace belong exporter; construction does not impose browser content model.

Other typed normal/specific functions retain the exact catalogs below. The thirteen void names are area,base,br,col,embed,hr,img,input,link,meta,source,track,wbr. Use one shared classification table, not a public per-node boolean or copied list. A typed void rejects any positional argument. Generic elem can construct children, but the exporter then reports exact `HTML void elements must not have children`; emit only the start tag, without slash/end tag.

## Exact specific attribute contracts

Int means full i64 unless explicitly positive/nonnegative; Float uses the measured numeric cast. Presence serializes true as empty and omits false. Lists preserve order and their declared space/comma delimiter, with scalar shorthand only where specified and no deduplication. Unknown names report `unexpected argument: NAME`; invalid types report `expected <type>, found <public type>` with long string/integer/boolean names. Positive values reject zero/negative with `number must be positive`; nonnegative values reject negatives with `number must be at least zero`. Enum language None stays distinct from the same-spelled Str.

Reusable target = _blank|_self|_parent|_top or freeStr. Referrerpolicy = language None→empty or no-referrer|no-referrer-when-downgrade|same-origin|origin|strict-origin|origin-when-cross-origin|strict-origin-when-cross-origin|unsafe-url. Rel scalar/space-list closed tokens alternate,canonical,author,bookmark,dns-prefetch,expect,external,help,icon,manifest,modulepreload,license,next,nofollow,noopener,noreferrer,opener,pingback,preconnect,prefetch,preload,prev,privacy-policy,search,stylesheet,tag,terms-of-service. Ping Str/space-list preserves empty, no space inside array item. URLs/MIME/lang strings remain free with no extra normalization/validation.

| Tag | Specific attributes |
|---|---|
| ol | reversed:Presence,start:Int,type:"1"\|"a"\|"A"\|"i"\|"I" |
| li | value:Int (including zero/negative) |
| a | download,href,hreflang,type:Str; ping; referrerpolicy; rel; target |
| br,wbr,caption,hr,table,tbody,tfoot,thead,tr | none |
| button | command:toggle-popover\|show-popover\|hide-popover\|close\|request-close\|show-modal orStr; commandfor,form,formaction,name,popovertarget,value:Str; disabled,formnovalidate:Presence; formenctype:application/x-www-form-urlencoded\|multipart/form-data\|text/plain; formmethod:GET\|POST\|dialog; formtarget:target; popovertargetaction:toggle\|show\|hide; type:submit\|reset\|button |
| col,colgroup | span:positiveInt |
| iframe | allow,src,srcdoc:Str; allowfullscreen:Presence; height,width:nonnegativeInt; loading:lazy\|eager; name:target; referrerpolicy;sandbox:closedspace-list |
| select | autocomplete:closedspace-list; disabled,multiple,required:Presence; form,name:Str; size:positiveInt |
| template | shadowrootclonable,shadowrootcustomelementregistry,shadowrootdelegatesfocus,shadowrootserializable:Presence; shadowrootmode:open\|closed |
| video | autoplay,controls,loop,muted,playsinline:Presence; crossorigin:anonymous\|use-credentials; height,width:nonnegativeInt; poster,src:Str; preload:language None\|Auto\|Str("metadata") |
| base | href:Str;target |
| blockquote,q | cite:Str |
| canvas | height,width:nonnegativeInt |
| data | value:Str |
| del,ins | cite:Str;datetime:DateTime |
| details | name:Str;open:Presence |
| dialog | open:Presence |
| embed | height,width:nonnegativeInt;src,type:Str |
| fieldset | disabled:Presence;form,name:Str |
| label | for:Str |
| map,slot | name:Str |
| meter | high,low,max,min,optimum,value:Float |
| optgroup | disabled:Presence;label:Str |
| option | disabled,selected:Presence;label,value:Str |
| output | for:Str\|space-list<Str>;form,name:Str |
| progress | max,value:Float |
| td | colspan:positiveInt;headers:Str\|space-list<Str>;rowspan:nonnegativeInt |
| th | abbr:Str;colspan:positiveInt;headers:Str\|space-list<Str>;rowspan:nonnegativeInt;scope:row\|col\|rowgroup\|colgroup |
| time | datetime:Duration\|DateTime |
| track | default:Presence;kind:subtitles\|captions\|descriptions\|chapters\|metadata;label,src,srclang:Str |
| area | alt,download,href:Str;coords:comma-list<Float> no shorthand;ping;referrerpolicy;rel;shape:circle\|default\|poly\|rect;target |
| audio | autoplay,controls,loop,muted:Presence;crossorigin:anonymous\|use-credentials;preload:language None\|Auto\|Str("metadata");src:Str |
| form | accept-charset:utf-8;action,name:Str;autocomplete:Bool→on\|off;enctype:three formenctype tokens;method:GET\|POST\|dialog;novalidate:Presence;rel;target |
| object | data,form,type:Str;height,width:nonnegativeInt;name:target |
| img | alt,src,usemap:Str;crossorigin;decoding:auto\|sync\|async;fetchpriority:auto\|high\|low;height,width:nonnegativeInt;ismap:Presence;loading:lazy\|eager;referrerpolicy;sizes:comma-list<source-size> no shorthand;srcset:comma-list<image-candidate> no shorthand |
| link | as:audio\|audioworklet\|document\|embed\|font\|frame\|iframe\|image\|json\|manifest\|object\|paintworklet\|report\|script\|serviceworker\|sharedworker\|style\|track\|video\|webidentity\|worker\|xslt;blocking:list tokenblocking;color:Color;crossorigin;disabled:Presence;fetchpriority:auto\|high\|low;href,hreflang,integrity,media,type:Str;imagesizes:list<source-size>;imagesrcset:list<image-candidate>;referrerpolicy;rel;sizes:space-list<icon-size> |
| source | height,width:nonnegativeInt;media,src,type:Str;sizes:list<source-size>;srcset:list<image-candidate> |
| meta | charset:utf-8;content,media,name:Str;http-equiv:content-type\|default-style\|refresh\|x-ua-compatible\|content-security-policy |
| script | async,defer,nomodule:Presence;blocking:list tokenblocking;crossorigin;fetchpriority:auto\|high\|low;integrity,src:Str;referrerpolicy;type:module or free Str;body:rawStr |
| style | blocking:list tokenblocking;media:Str;body:rawStr |

Sandbox closed tokens: allow-downloads,allow-forms,allow-modals,allow-orientation-lock,allow-pointer-lock,allow-popups,allow-popups-to-escape-sandbox,allow-presentation,allow-same-origin,allow-scripts,allow-top-navigation,allow-top-navigation-by-user-activation,allow-top-navigation-to-custom-protocols.

Autocomplete closed scalar/list tokens: shipping,billing,name,honorific-prefix,given-name,additional-name,family-name,honorific-suffix,nickname,username,new-password,current-password,one-time-code,organization-title,organization,street-address,address-line1,address-line2,address-line3,address-level4,address-level3,address-level2,address-level1,country,country-name,postal-code,cc-name,cc-given-name,cc-additional-name,cc-family-name,cc-number,cc-exp,cc-exp-month,cc-exp-year,cc-csc,cc-type,transaction-currency,transaction-amount,language,bday,bday-day,bday-month,bday-year,sex,url,photo,home,work,mobile,fax,pager,tel,tel-country-code,tel-national,tel-area-code,tel-local,tel-local-prefix,tel-local-suffix,tel-extension,email,impp.

Input exactly: accept Str|comma-list Str; alpha,checked,disabled,formnovalidate,multiple,readonly,required:Presence; alt,dirname,form,formaction,list,name,pattern,placeholder,popovertarget,src:Str; autocomplete; colorspace:limited-srgb|display-p3; formenctype three form tokens/formmethod GET|POST|dialog/formtarget; height,maxlength,minlength,width:nonnegativeInt,size:positiveInt; max,min:Str|Float|DateTime; step:positiveFloat|any; popovertargetaction:toggle|show|hide; type:hidden|text|search|tel|url|email|password|date|month|week|time|datetime-local|number|range|color|checkbox|radio|file|submit|image|reset|button; value:Str|Float|DateTime|Color|comma-list Str.

Textarea exactly autocomplete; cols,rows:positiveInt; maxlength,minlength:nonnegativeInt; disabled,readonly,required:Presence; dirname,form,name,placeholder:Str; wrap:soft|hard.

DateTime serializes as YYYY-MM-DD, HH:MM[:SS], or YYYY-MM-DDTHH:MM[:SS]. Duration emits ordered nonzero Nw Nd Nh Nm Ns components, or 0s when zero. Color uses the current public CSS form. Icon size is two nonnegative Ints rendered WxH. An image-candidate Dict requires nonempty src:Str and at most one positive width:Int or density:Float, yielding SRC, SRC Nw or SRC Nd; reject empty src and leading/trailing comma. A source-size Dict(condition:Str,size:Length) yields (CONDITION) CSS-SIZE. Invent no extra dictionary keys or ambiguous casts.

Preload accepts exactly Value::None→"none", Value::Auto→"auto", Value::Str("metadata")→"metadata". Strings "none"/"auto" reject with exact `expected none, auto, or "metadata"`. This differs from referrerpolicy None→empty. Preserve all 76 globals and the 48 specific attributes in the seven-tag signatures, plus every remaining cast; add no spelling-based coercion.

## Frame, composition and verification boundaries

Frame is separate from the typed catalog. It takes exactly one required positional Content and no named arguments: missing `missing argument: body`, named ``the argument `body` is positional``, excess `unexpected argument`. Produce Content::HtmlFrame/repr frame, not HtmlElem, a literal sentinel, SVG string or collapsed body. Paged consumption is transparent; HTML produces an SVG island through the respective owners. No browser/CSS/MathML/media/network/other-tag behavior follows from this façade.

Reflection lists specifics before globals; actual attrs/repr/DOM retain named occurrence order after omissions. Exporter ownership includes escaping, nesting, shared void classification and independent raw/document/phrasing/whitespace behavior. Measured normal/block/inline/ruby/datalist behavior and boundary whitespace corrections belong to `00_nucleo/prompts/infra/export/html.md` and descendants. Constructors do not create per-tag exporter branches. Older scope-outs denying later tags/frame are historical; browser/context/content-model/CSS/MathML/position gaps remain separate.

Keep the direct native_html_frame pub(in crate::compiler) reexport before inline tests. Do not move its constructor, alter exports/tables or add a wrapper. For html_constructor_surface_tests/html_frame_tests remove only the parent outer cfg(test), retaining inner #![cfg(test)], path/name/visibility/child ownership; the full module stays absent in production. Preserve test bodies, dependencies, logic, public defaults and phase.

Acceptance covers all 112 tags/specific casts, all 76 globals with invalid/unset/order/None-versus-string/Presence/empty-list controls, strict normal/void/raw bodies, all thirteen void classifications, root/std/alias/With identities, all 114 format identity pairs, four feature/target quadrants and eight profiles, gated lookup/errors/hints/spans, and real DOM escaping/nesting/raw/frame output. Unknown, crash, timeout, ambiguous cast or unobserved DOM fails closed. Preserve independent fixtures/correctors; direct Rust reconstruction cannot hide a changed public signature. Require build/full workspace/core/test inventory, strict Clippy/Rustdoc/fmt, strict/reverse lineage, valid pins/V15/V26 and restricted resealing. No production, render-speed or global-parity claim follows.

Apply pinned format-native-identity nucleus. Native/Format grouping is justified by same payload/ABI for projection/execution, but test every alternative for name/namespace/pointer/Eq/Hash/clone/With/dispatch; inventory/link V19 to tests without suppression, explicitly name new variant in exhaustive matches. No external definition/signature/field/trait/compatibility/default/phase changes, ADR-0127 continuous flow. Before code functional RED via current factories for fifteen cases and real observation against unmarked Native; GREEN full corpus/144 field-call/456 compatibility outside where/kind, public-contract and sizeof before/after. L0/ownership/V26 before restricted eleven-pair reseal; build/workspace/strict Clippy/Rustdoc/fmt/strict-reverse lineage/global no new errors/warnings. Sequential 12GiB/High10GiB/swap0/jobs1. Only where/kind admission closes; full selection/show/query/synthesis/carrier morphology/counter, FG1/RM1/SY1/RG1/V16 retain separate scopes/gates. No old-candidate copying or publication.
