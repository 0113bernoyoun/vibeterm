# VibeTerm

**몰입을 끊지 마세요.**

![Version](https://img.shields.io/badge/version-0.4.0-blue)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey)
![Rust](https://img.shields.io/badge/rust-stable-orange)

[English](./README.md)

## 왜 VibeTerm인가?

코딩에 완전히 몰입한 순간. 아이디어가 물 흐르듯 나오는 그 타이밍. 앱을 전환하고, 창을 찾고, 컨텍스트를 바꾸느라 흐름이 끊기는 건 최악입니다.

**VibeTerm은 바이브 코더를 위해 만들어졌습니다** — 흐름을 유지하며 마찰 없이 일하고 싶은 개발자를 위한 터미널. 하나의 터미널에서 모든 것을 해결하세요.

- **전환 말고, 분할하세요.** 여러 터미널을 나란히 열어두세요. ⌘+Tab 지옥은 이제 그만.
- **파일도 바로 여기서.** 터미널을 벗어나지 않고 파일을 탐색하고 열 수 있습니다.
- **나만의 워크스페이스.** 탭, 패널, 테마 — 내 생각의 흐름대로 구성하세요.
- **네이티브 & 빠름.** Rust로 제작. macOS의 일부처럼 자연스럽습니다.

```
+------------------------------------------+
| [macOS 네이티브 메뉴바]                    |
+------------------------------------------+
|  [탭 바]      ▶1 shell │ 2 file.rs   +   |
+--------+---------------------------------+
| [사이드 |   [터미널 영역]                  |
|   바]  |   +-------------+-------------+ |
|        |   |  패널 1    ║  패널 2      | |
| 파일   |   |  (포커스)  ║             | |
|        |   +============+-------------+ |
|  트리  |   디바이더 (드래그 가능)         |
+--------+---------------------------------+
|  [상태 바]  VibeTerm │ 패널: 2           |
+------------------------------------------+
```

## 기능

### 몰입 유지
- 수평 분할로 워크스페이스 확장 (Cmd+D)
- 클릭으로 포커스 전환
- 드래그로 패널 크기 조절

### 모든 것을 한 곳에서
- 사이드바에 통합된 파일 탐색기
- 여러 컨텍스트를 위한 멀티 탭
- macOS 네이티브 메뉴바

### 속도를 위해 설계됨
- Alacritty 기반 터미널 백엔드
- ANSI 이스케이프 시퀀스 완벽 지원
- 비동기 PTY 통신

### 나만의 스타일로
- 다크 브라운 테마 (완전 커스터마이징 가능)
- CJK 폰트 지원 (한글/일본어/중국어)
- IME 입력 지원

## 설치

### 요구사항
- macOS (Apple Silicon / Intel)
- Rust (Stable)

### 빌드

```bash
git clone https://github.com/0113bernoyoun/vibeterm.git
cd vibeterm
cargo build --release
cargo run --release
```

## 키보드 단축키

| 단축키 | 기능 |
|--------|------|
| `Cmd+T` | 새 탭 |
| `Cmd+W` | 현재 패널/탭 닫기 |
| `Cmd+D` | 수평 분할 |
| `Cmd+B` | 사이드바 토글 |
| `Cmd+,` | 설정 창 |
| `Cmd+1-9` | 탭 전환 |
| `Ctrl+Tab` | 다음 패널 |
| `Ctrl+Shift+Tab` | 이전 패널 |

## 설정

설정 파일: `~/.config/vibeterm/config.toml`

```toml
[theme]
background = "#2E1A16"
surface = "#3A241E"
primary = "#E07A5F"
text = "#F4F1DE"
text_dim = "#A0968A"
border = "#4A2E28"

[font]
family = "JetBrains Mono"
size = 13.0
```

## 기술 스택

| 구성요소 | 라이브러리 | 버전 |
|---------|-----------|------|
| 언어 | Rust | Stable |
| GUI | egui + eframe | 0.31 |
| 터미널 | egui_term | 0.1 |
| 메뉴 | muda | 0.15 |
| 설정 | serde + toml | 1.0 / 0.8 |
| 비동기 | tokio | 1.0 |

## 프로젝트 구조

```
src/
├── main.rs          # 진입점
├── app.rs           # 메인 애플리케이션 상태
├── config.rs        # 설정 관리
├── menu.rs          # 네이티브 메뉴바
├── theme.rs         # 테마 시스템, CJK 폰트
└── ui/
    ├── mod.rs
    ├── tab_bar.rs   # 탭바 컴포넌트
    ├── sidebar.rs   # 파일 탐색기
    └── status_bar.rs
```

## 알려진 제한사항

- **한글 IME**: winit/egui의 IME 지원 한계로 인해 일부 환경에서 한글 입력이 불완전할 수 있습니다.
  - [winit#1497](https://github.com/rust-windowing/winit/issues/1497)
  - [egui#248](https://github.com/emilk/egui/issues/248)

## 로드맵

- [ ] 수직 분할 (Cmd+Shift+D)
- [ ] 스크롤백 버퍼
- [ ] 텍스트 선택 및 복사
- [ ] 탭 드래그 앤 드롭 재정렬
- [ ] 새 창 열기 (Cmd+Shift+N)
- [ ] 바이브 코딩을 위한 AI 통합

## 라이선스

MIT License

## 기여

Pull request를 환영합니다. 큰 변경사항의 경우, 먼저 이슈를 열어 변경하고자 하는 내용을 논의해주세요.
