# Registro de Alterações (Changelog)

Todas as alterações notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Semantic Versioning](https://semver.org/lang/pt-BR/).

---

## [0.1.0] - 2026-11-17

### ✈️ Renascimento Comunitário (Community Revival)

Primeira versão da nova fase do projeto sob a governança da comunidade **RicardTech**.

#### Adicionado
- **Empacotamento Arch Linux (AUR):** Criada a receita oficial `packaging/arch/PKGBUILD` para o pacote `paperplane-next-git`.
- **Localização 100% em Português do Brasil (PT-BR):** Catálogo de tradução `po/pt_BR.po` verificado e completo (310 mensagens traduzidas).
- **Redundância Git Dual-Push:** Sincronização automática simultânea entre **GitHub** (`ricardtech/paperplane-next`) e **GitLab** (`ricard-tech/paperplane-next`).
- **Licença em Português:** Adicionado o documento `LICENCA.pt_BR.md` com a versão da GNU GPLv3 para o público lusófono.
- **Painel em Tempo Real:** Integração com o Live Progress Dashboard para monitoramento transparente das fases de engenharia.

#### Alterado
- **Atualização da Toolchain:** Validação e compatibilidade com **Rust 1.80+ / 1.100**, **Meson 1.12+** e **Ninja 1.13+**.
- **Novo README:** Documentação modernizada com badges de status, instruções de instalação para distros baseadas em Arch Linux e guia de contribuição.
- **Mapeamento de Dependências:** Mapeamento explícito das bibliotecas nativas de aceleração gráfica e animações (`rlottie` e `telegram-tdlib` / `libtdjson`).
