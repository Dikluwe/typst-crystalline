# P1339 — acesso de teste à identidade real de StyleChain

Extensão mecânica de instrumentação, anterior à sua implementação.
Autor da integração: `/root`; observer: `/root/p1312_review`.
Executado sem atestação de isolamento. Autorização do dono: corrigir os testes.

Medição anterior: `entities/style_chain.rs:274-293` define StyleNode privado
e StyleChain com Option<Arc<StyleNode>>, Clone O(1). O L0 vigente
`prompts/entities/style_chain.md`, SHA-256
`1efe881899ab5cdc290be6350e2da6f5e3ea8cbff60ab4e65e01585ec28cdf6c`,
declara essa representação e exige verificar a preservação pelo Clone.
O parecer independente `p1339-verifier-style-r3-engineering-review-r1.json`,
SHA-256 `85c08dd60780f10d2228f89af73ebc07ce09ded31c4b1185b6e1274a22ab424a`,
demonstrou que o DTO anterior copiava a identidade da captura em vez de
medir o recurso efetivamente usado no replay. O teste rodou, mas não provou
essa obrigação de F. Seu resultado histórico não é apagado.

Adicionar no próprio owner StyleChain somente accessor pub(crate), sob
`#[cfg(p1339_observation)]`, que retorna Option<usize> com o endereço real
do Arc backing. None significa cadeia vazia canônica, nunca endereço fictício.
O accessor não existe no build produtivo, não muda representação, resolução,
defaults, API externa ou qualquer byte normativo do L0.

Observer registra clone da StyleChain efetiva da transaction imediatamente
antes do dispatch, conserva esse recurso para impedir reutilização do endereço
e deriva identidade e propriedades desse clone real. Captura gravada e replay
são observados separadamente, sem copiar a identidade esperada, derivar ID
de tamanho/hash ou apontar para Engine temporário. Somente a telemetria muda.

Esta extensão acrescenta um consumer mecânico de instrumentação ao inventário
de integração, mantendo ownership 1:1. Pins do source antes/depois e cadeia
transitiva de includes devem integrar os novos recibos. Rerodar teste focal e
submeter ligação efetiva ao verificador. Nenhum crédito F antecipado e nenhuma
alteração de expectativas, contrato semântico, selo histórico ou budget.
