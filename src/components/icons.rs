use leptos::*;

#[component]
pub fn FullCrossIcon() -> impl IntoView {
    view! {
        <svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M27.681 1.63437C26.5094 0.462798 24.6099 0.4628 23.4383 1.63437L16 9.07271L8.56166 1.63437C7.39009 0.462798 5.49059 0.4628 4.31902 1.63437L1.63437 4.31902C0.462799 5.49059 0.462801 7.39009 1.63437 8.56166L9.07271 16L1.63437 23.4383C0.462798 24.6099 0.4628 26.5094 1.63437 27.681L4.31902 30.3656C5.49059 31.5372 7.39009 31.5372 8.56166 30.3656L16 22.9273L23.4383 30.3656C24.6099 31.5372 26.5094 31.5372 27.681 30.3656L30.3656 27.681C31.5372 26.5094 31.5372 24.6099 30.3656 23.4383L22.9273 16L30.3656 8.56166C31.5372 7.39009 31.5372 5.49059 30.3656 4.31902L27.681 1.63437Z" fill="#31C3BD"/>
        </svg>
    }
}

#[component]
pub fn FullCircleIcon() -> impl IntoView {
    view! {
        <svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M31.9704 15.8706C31.9704 7.10551 24.8649 0 16.0998 0C7.33476 0 0.229248 7.10551 0.229248 15.8706C0.229248 24.6357 7.33476 31.7412 16.0998 31.7412C24.8649 31.7412 31.9704 24.6357 31.9704 15.8706ZM9.63405 15.8706C9.63405 12.2996 12.5289 9.4048 16.0998 9.4048C19.6708 9.4048 22.5656 12.2996 22.5656 15.8706C22.5656 19.4416 19.6708 22.3364 16.0998 22.3364C12.5289 22.3364 9.63405 19.4416 9.63405 15.8706Z" fill="#F2B137"/>
        </svg>

    }
}

#[component]
pub fn OutlineCrossIcon() -> impl IntoView {
    view! {
            <svg width="64" height="64" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
    <g id="Combined Shape Copy 2">
    <mask id="path-1-inside-1_0_1844" fill="white">
    <path fill-rule="evenodd" clip-rule="evenodd" d="M53.2406 1.14742C52.0691 -0.0241513 50.1696 -0.0241497 48.998 1.14742L32 18.1454L15.002 1.14742C13.8304 -0.0241506 11.9309 -0.0241496 10.7594 1.14742L1.14742 10.7594C-0.0241499 11.9309 -0.0241481 13.8304 1.14742 15.002L18.1454 32L1.14742 48.998C-0.0241506 50.1696 -0.0241496 52.0691 1.14742 53.2406L10.7594 62.8526C11.9309 64.0241 13.8304 64.0241 15.002 62.8526L32 45.8546L48.998 62.8526C50.1696 64.0242 52.0691 64.0241 53.2406 62.8526L62.8526 53.2406C64.0242 52.0691 64.0241 50.1696 62.8526 48.998L45.8546 32L62.8526 15.002C64.0242 13.8304 64.0241 11.9309 62.8526 10.7594L53.2406 1.14742Z"/>
    </mask>
    <path d="M48.998 1.14742L47.5838 -0.266791L47.5838 -0.26679L48.998 1.14742ZM53.2406 1.14742L51.8264 2.56163V2.56164L53.2406 1.14742ZM32 18.1454L30.5858 19.5596L32 20.9738L33.4142 19.5596L32 18.1454ZM15.002 1.14742L13.5878 2.56164L13.5878 2.56164L15.002 1.14742ZM10.7594 1.14742L12.1736 2.56164H12.1736L10.7594 1.14742ZM1.14742 10.7594L2.56164 12.1736V12.1736L1.14742 10.7594ZM1.14742 15.002L-0.266789 16.4162L-0.266789 16.4162L1.14742 15.002ZM18.1454 32L19.5596 33.4142L20.9738 32L19.5596 30.5858L18.1454 32ZM1.14742 48.998L-0.266791 47.5838L-0.266792 47.5838L1.14742 48.998ZM1.14742 53.2406L2.56164 51.8264H2.56164L1.14742 53.2406ZM10.7594 62.8526L9.34515 64.2668H9.34515L10.7594 62.8526ZM15.002 62.8526L16.4162 64.2668H16.4162L15.002 62.8526ZM32 45.8546L33.4142 44.4404L32 43.0262L30.5858 44.4404L32 45.8546ZM48.998 62.8526L50.4122 61.4384V61.4384L48.998 62.8526ZM53.2406 62.8526L54.6549 64.2668H54.6549L53.2406 62.8526ZM62.8526 53.2406L61.4384 51.8264H61.4384L62.8526 53.2406ZM62.8526 48.998L64.2668 47.5838V47.5838L62.8526 48.998ZM45.8546 32L44.4404 30.5858L43.0262 32L44.4404 33.4142L45.8546 32ZM62.8526 15.002L61.4384 13.5878L61.4384 13.5878L62.8526 15.002ZM62.8526 10.7594L64.2668 9.34515V9.34515L62.8526 10.7594ZM50.4122 2.56164C50.8027 2.17111 51.4359 2.17111 51.8264 2.56163L54.6549 -0.26679C52.7022 -2.21942 49.5364 -2.21941 47.5838 -0.266791L50.4122 2.56164ZM33.4142 19.5596L50.4122 2.56164L47.5838 -0.26679L30.5858 16.7312L33.4142 19.5596ZM13.5878 2.56164L30.5858 19.5596L33.4142 16.7312L16.4162 -0.266791L13.5878 2.56164ZM12.1736 2.56164C12.5641 2.17111 13.1973 2.17111 13.5878 2.56164L16.4162 -0.266791C14.4636 -2.21941 11.2978 -2.21941 9.34515 -0.26679L12.1736 2.56164ZM2.56164 12.1736L12.1736 2.56164L9.34515 -0.26679L-0.266791 9.34515L2.56164 12.1736ZM2.56164 13.5878C2.17111 13.1973 2.17111 12.5641 2.56164 12.1736L-0.266791 9.34515C-2.21941 11.2978 -2.21941 14.4636 -0.266789 16.4162L2.56164 13.5878ZM19.5596 30.5858L2.56164 13.5878L-0.266789 16.4162L16.7312 33.4142L19.5596 30.5858ZM2.56164 50.4122L19.5596 33.4142L16.7312 30.5858L-0.266791 47.5838L2.56164 50.4122ZM2.56164 51.8264C2.17111 51.4359 2.17111 50.8027 2.56164 50.4122L-0.266792 47.5838C-2.21941 49.5364 -2.21941 52.7022 -0.266788 54.6549L2.56164 51.8264ZM12.1736 61.4384L2.56164 51.8264L-0.26679 54.6549L9.34515 64.2668L12.1736 61.4384ZM13.5878 61.4384C13.1973 61.8289 12.5641 61.8289 12.1736 61.4384L9.34515 64.2668C11.2978 66.2194 14.4636 66.2194 16.4162 64.2668L13.5878 61.4384ZM30.5858 44.4404L13.5878 61.4384L16.4162 64.2668L33.4142 47.2688L30.5858 44.4404ZM50.4122 61.4384L33.4142 44.4404L30.5858 47.2688L47.5838 64.2668L50.4122 61.4384ZM51.8264 61.4384C51.4359 61.8289 50.8027 61.8289 50.4122 61.4384L47.5838 64.2668C49.5364 66.2194 52.7022 66.2194 54.6549 64.2668L51.8264 61.4384ZM61.4384 51.8264L51.8264 61.4384L54.6549 64.2668L64.2668 54.6549L61.4384 51.8264ZM61.4384 50.4122C61.8289 50.8027 61.8289 51.4359 61.4384 51.8264L64.2668 54.6549C66.2194 52.7022 66.2194 49.5364 64.2668 47.5838L61.4384 50.4122ZM44.4404 33.4142L61.4384 50.4122L64.2668 47.5838L47.2688 30.5858L44.4404 33.4142ZM61.4384 13.5878L44.4404 30.5858L47.2688 33.4142L64.2668 16.4162L61.4384 13.5878ZM61.4384 12.1736C61.8289 12.5641 61.8289 13.1973 61.4384 13.5878L64.2668 16.4162C66.2194 14.4636 66.2194 11.2978 64.2668 9.34515L61.4384 12.1736ZM51.8264 2.56164L61.4384 12.1736L64.2668 9.34515L54.6549 -0.266792L51.8264 2.56164Z" fill="#31C3BD" mask="url(#path-1-inside-1_0_1844)"/>
    </g>
    </svg>
        }
}

#[component]
pub fn OutlineCircleIcon() -> impl IntoView {
    view! {
    <svg width="66" height="66" viewBox="0 0 66 66" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path id="Oval Copy" d="M33 2C50.1208 2 64 15.8792 64 33H66C66 14.7746 51.2254 0 33 0V2ZM2 33C2 15.8792 15.8792 2 33 2V0C14.7746 0 0 14.7746 0 33H2ZM33 64C15.8792 64 2 50.1208 2 33H0C0 51.2254 14.7746 66 33 66V64ZM64 33C64 50.1208 50.1208 64 33 64V66C51.2254 66 66 51.2254 66 33H64ZM33 18.963C25.2476 18.963 18.963 25.2476 18.963 33H20.963C20.963 26.3521 26.3521 20.963 33 20.963V18.963ZM47.037 33C47.037 25.2476 40.7524 18.963 33 18.963V20.963C39.6479 20.963 45.037 26.3521 45.037 33H47.037ZM33 47.037C40.7524 47.037 47.037 40.7524 47.037 33H45.037C45.037 39.6479 39.6479 45.037 33 45.037V47.037ZM18.963 33C18.963 40.7524 25.2476 47.037 33 47.037V45.037C26.3521 45.037 20.963 39.6479 20.963 33H18.963Z" fill="#F2B137"/>
    </svg>

        }
}

#[component]
pub fn RedoIcon() -> impl IntoView {
    view! {
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
    <g id="Redo">
    <path id="Path" d="M19.5241 2.75843e-07H17.644C17.3812 -0.000279724 17.1679 0.21264 17.1676 0.47564C17.1676 0.48336 17.1678 0.49108 17.1681 0.4988L17.3268 3.78292C15.46 1.58176 12.7198 0.31428 9.83484 0.31744C4.41536 0.31748 -0.00395734 4.74324 2.65924e-06 10.1663C0.00396266 15.598 4.40584 20 9.83484 20C12.2702 20.0034 14.6195 19.0993 16.425 17.4639C16.6208 17.2885 16.6375 16.9874 16.4622 16.7915C16.4563 16.7849 16.4502 16.7785 16.444 16.7722L15.0957 15.423C14.9186 15.2459 14.6346 15.2363 14.4461 15.4012C11.5521 17.949 7.14188 17.6669 4.59564 14.7709C2.0494 11.875 2.3314 7.46188 5.22544 4.914C8.11948 2.36612 12.5297 2.64828 15.0759 5.54424C15.2755 5.77124 15.4601 6.01096 15.6287 6.26188L11.6024 6.06864C11.3398 6.05616 11.1169 6.25896 11.1044 6.52168C11.104 6.5294 11.1038 6.53712 11.1039 6.54484V8.4262C11.1039 8.6892 11.3169 8.9024 11.5798 8.9024H19.5242C19.787 8.9024 20 8.6892 20 8.4262V0.4762C20 0.2132 19.787 2.75843e-07 19.5241 2.75843e-07Z" fill="#1F3641"/>
    </g>
    </svg>

        }
}
