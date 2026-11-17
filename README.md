<h1 align="center">
  <img src="data/icons/app.drey.PaperPlane.svg" alt="PaperPlane Next" width="160" height="160"/>
  <br>
  PaperPlane Next
</h1>

<p align="center">
  <strong>Cliente Telegram moderno, ultra-leve e nativo em Rust + GTK4 para Linux</strong>
  <br>
  <em>(Continuação Comunitária &bull; Community Revival Edition)</em>
</p>

<p align="center">
  <a href="https://github.com/ricardtech/paperplane-next"><img src="https://img.shields.io/badge/GitHub-ricardtech%2Fpaperplane--next-181717?logo=github" alt="GitHub Repo"/></a>
  <a href="https://gitlab.com/ricard-tech/paperplane-next"><img src="https://img.shields.io/badge/GitLab-ricard--tech%2Fpaperplane--next-FC6D26?logo=gitlab" alt="GitLab Repo"/></a>
  <a href="https://aur.archlinux.org/packages/paperplane-next-git"><img src="https://img.shields.io/badge/Arch_AUR-paperplane--next--git-1793D1?logo=arch-linux" alt="Arch AUR"/></a>
  <img src="https://img.shields.io/badge/Rust-1.80+-DEA584?logo=rust" alt="Rust"/>
  <img src="https://img.shields.io/badge/GTK4-Libadwaita-4A90D9?logo=gnome" alt="GTK4"/>
  <img src="https://img.shields.io/badge/PT--BR-100%25_Traduzido-009c3b" alt="PT-BR"/>
</p>

<br>

<p align="center">
  <img width="650" src="data/resources/screenshots/screenshot1.png" alt="Interface do PaperPlane Next"/>
</p>

---

## ✈️ Sobre o PaperPlane Next

O **PaperPlane Next** é a continuação comunitária do projeto PaperPlane, mantido e atualizado para oferecer a melhor e mais leve experiência de Telegram no desktop Linux.

Enquanto clientes baseados em Electron ou Qt pesado podem demorar para abrir e consumir centenas de megabytes de memória, o **PaperPlane Next** utiliza **Rust**, a biblioteca nativa **GTK4 / Libadwaita** e a **TDLib** oficial do Telegram para entregar:

* ⚡ **Inicialização Instantânea:** Abre no clique em frações de segundo.
* 🪶 **Consumo Mínimo de RAM:** Média de 40MB a 80MB de memória RAM em uso ativo.
* 🎨 **Visual 100% Nativo GNOME:** Compatível com os padrões de design Libadwaita, cantos arredondados, alternância automática de tema claro/escuro e gestos fluidos de trackpad.
* 🇧🇷 **Localização Completa em Português do Brasil:** 100% das strings da interface e menus traduzidos para PT-BR.
* 🔐 **Segurança e Estabilidade:** Baseado na biblioteca oficial **TDLib** (*Telegram Database Library*), que garante criptografia nativa MTProto e cache local eficiente.

---

## 📦 Como Instalar

### 1. No Arch Linux, Manjaro ou EndeavourOS (via AUR)

O pacote oficial do projeto está disponível no AUR como `paperplane-next-git`:

```bash
# Usando o yay:
yay -S paperplane-next-git

# Ou usando o paru:
paru -S paperplane-next-git
```

*Também pode ser instalado com um clique através do **Pamac** ("Adicionar ou Remover Programas") ativando o suporte a AUR nas preferências.*

---

### 2. Compilação Manual a partir do Código Fonte

#### Pré-requisitos (Arch Linux):
```bash
sudo pacman -S --needed git meson ninja rust cargo gtk4 libadwaita libshumate telegram-tdlib rlottie
```

#### Passo a passo:
```bash
# 1. Clone o repositório
git clone https://github.com/ricardtech/paperplane-next.git
cd paperplane-next

# 2. Configure o build com o Meson
meson setup build --prefix=/usr

# 3. Compile o projeto
meson compile -C build

# 4. Instale no sistema
sudo meson install -C build
```

---

## 🗺️ Roadmap de Evolução

- [x] Criação da fundação comunitária e espelhamento redundante (GitHub + GitLab).
- [x] Auditoria de dependências com Rust 1.80+ e Meson.
- [x] Tradução completa em Português do Brasil (`pt_BR.po`).
- [x] Estruturação da receita PKGBUILD para o Arch Linux AUR.
- [ ] Atualização contínua das structs de eventos da TDLib (Tópicos de Fóruns e Reações).
- [ ] Pipeline automatizado de CI/CD via GitHub Actions e GitLab CI para compilação de binários estáticos / Flatpak.

---

## 🤝 Como Contribuir

Contribuições são muito bem-vindas! Se você desenvolve em Rust, é designer ou tradutor:

1. Faça um Fork do projeto no GitHub ou GitLab.
2. Crie uma branch para sua funcionalidade (`git checkout -b feature/minha-melhoria`).
3. Commit suas alterações seguindo o padrão Conventional Commits (`git commit -m 'feat: adicionar suporte a reações'`).
4. Envie o push para sua branch (`git push origin feature/minha-melhoria`).
5. Abra um Pull Request ou Merge Request.

---

## 📄 Licença

Este projeto é software livre licenciado sob a licença [GPLv3](COPYING).
Originalmente criado pela equipe Paper Plane Developers e mantido com carinho pela comunidade.
