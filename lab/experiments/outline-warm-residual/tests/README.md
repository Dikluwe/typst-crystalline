C executa os testes na RAM `/dev/shm/tekt-outline-warm-20261106`, preservando a receita `.cargo` completa.

```text
PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s <E>/tests -p test_checkers.py
python3 <E>/tests/check_phases.py <RAM>/case/phases.tsv
python3 <E>/tests/check_measurements.py <RAM>/measurements.csv --cycles 6 --variant base --variant joint --ram-dir <RAM>
rustc --edition=2021 --test <E>/tests/probe_contract.rs -C llvm-args=-align-all-functions=3 -o <RAM>/probe_contract
<RAM>/probe_contract --test-threads=1
```

Ao copiar E para RAM, preservar o sibling `joint-font-witness/tests/check_phases.py` ou definir `TEKT_OUTLINE_PHASE_ORACLE` para sua cópia read-only. O checker importa esse oracle privado, conservando as regras antigas.

`check_phases.py`: TSV pid/cycle/label/calls/ns. Verifica pass1..8/outcome, até oito passes por helper agregado, context.attempt1..5, pais inclusivos e fases grossas dentro de paged. `font.*` deve reproduzir exatamente calls/ns de initialize e caber no estágio qualificado. Esses valores duplicam initialize: não somá-los de novo. `font.outside` e `source.construct` não exigem pai. Source construction inclui construção+parse nos pontos L3 acordados; não é parse puro, nem possui contagem fixa em cache hits. Não subtrair fontes de um estágio sem atribuição qualificada.

`check_measurements.py`: campos anteriores mais cycle, em qualquer ordem sem duplicatas. Valida processos completos, CLI/startup0, warm1..2N, odd=same_bytes/even=main_edit, pareamento, wait4 e CPU single-core com tolerância de 1 ms. `sample_epoch` é timestamp. Marcador esperado: `TEKT_EPOCH_{(cycle//2)%2}`. Com `--ram-dir`, extrai PDFs offline via pdftotext nos snapshots cli.pdf/startup.pdf/cycle-001-same_bytes.pdf etc.; exige só o marcador esperado. Sem essa opção, declara marcadores não verificados. Repetição CPU/RSS final por linha não é um valor por ciclo para somar.

`probe_contract.rs` usa `#[path="../phase_probe.rs"]`: subprocessos on/off para OnceLock, nested restore, unwind, worker outside, atribuição exata, retorno Err por RAII, mark/count/zero e flush que drena. Sem sleeps ou limiar de duração. Usa evento unitário `event.collector.control` e scopes propositalmente aninhados; seu TSV testa API do coletor e não representa pipeline real para o checker de fases grossas.

`test_checkers.py` contém fixtures normais/adversárias. Snapshots unitários são saídas textuais controladas do extrator, não PDFs simuladamente certificados. Mutações úteis para C: inverter/não alternar marcador; trocar ciclo/PID; remover pass/outcome; pass9/attempt6; duplicar/remover atribuição; perder restore TLS/unwind ou herdar estágio no worker; duração font acima do estágio; CPU acima de wall/wait4. O gate de publicação do driver precisa de controle próprio com assinatura antiga/EOF incompleto: um PDF antigo no same_bytes conserva marcador correto e não é detectável apenas pela extração offline.

Canal físico: `physical.candidate` fica em pipeline.context; step.discovery/replay fica no candidate e layout.discovery/replay no step do mesmo papel. Complete+pending deve igualar a soma das chamadas reais physical.layout.*. Não há limite global de oito nem proporção imposta entre steps e layouts: replay/running requests não seguem necessariamente callbacks.passes. Esses scopes são inclusivos; fontes continuam qualificadas como font.context, sem atribuição inventada por papel físico. Os testes incluem ausência de outcome/pai, duração acima do pai e duplicação de label de papel.
