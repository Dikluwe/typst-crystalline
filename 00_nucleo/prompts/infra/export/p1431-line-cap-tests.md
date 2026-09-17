# Testes independentes P1431 — line caps raster e PDF

## Dono test-only

Consumer único: `03_infra/src/export/p1431_line_cap_tests.rs`.

## Obrigação

Gerar linhas `Butt`, `Round` e `Square` pela fronteira pública de `math.mat`,
isolar o item transportado e observar os exporters reais. No raster tiny-skia,
os endpoints distinguem as três áreas cobertas e o augment automático coincide
com um stroke `Square` equivalente. No PDF, os caminhos top-level e local
emitem `1 J` para Round e `2 J` para Square dentro de `q/Q`; Butt omite `0 J`.
Uma linha legada Butt conserva bytes determinísticos e não recebe operador
redundante.

Stream, pixel ou identidade de linha ausente/truncado/não atribuível é
`UNKNOWN_EXPORT_OBSERVATION`; nunca recebe aceitação ou crédito.
