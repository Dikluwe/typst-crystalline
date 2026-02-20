#!/bin/bash
# 🧊 Scaffolder da Arquitetura Cristalina (Tekt)

echo "💎 Inicializando Lattice da Arquitetura Cristalina..."

# Cria as restrições físicas (pastas)
mkdir -p 00_nucleo/{specs,contracts,adr}
mkdir -p 01_core/{domain,use_cases}
mkdir -p 02_shell/http/api/controllers
mkdir -p 03_infra/{database,cryptography,http_clients}
mkdir -p 04_wiring/
mkdir -p 10_bedrock/
mkdir -p 11_tools/
mkdir -p 20_lab/

echo "✔️  Pastas criadas."

# Confirma que os arquivos de defesa de IA existem e os lista
if [ -f ".agentrules" ]; then
    echo "✔️  .agentrules detectado. Instruções para LLMs (CLI/Aider) prontas."
else
    echo "⚠️  .agentrules não encontrado no diretório atual."
fi

if [ -f ".cursorrules" ]; then
    echo "✔️  .cursorrules detectado. Instruções para Cursor IDE prontas."
else
    echo "⚠️  .cursorrules não encontrado no diretório atual."
fi

# Deixa um aviso amigável no núcleo
cat > 00_nucleo/README.md <<'EOF'
# L0 (A Semente)
Nenhuma linha de código fonte executável deve existir aqui.
Somente Markdown, Interfaces, Contratos e Textos.
Se a sua IA tentar escrever lógicas (if/else) aqui, ela falhou.
EOF

echo ""
echo "🚀 Tekt inicializada! Lembre-se de configurar sua IA lendo o HOW_TO_IMPLEMENT.md."
