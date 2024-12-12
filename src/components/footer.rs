use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer
	class="flex flex-col items-start py-32 px-8 bg-zinc-800 text-white gap-16"
>
	<img src="/public/img/app.svg" alt="FuelEV" class="h-12" />
	<div
		class="items-center w-full flex flex-col lg:flex-row border border-white lg:border-l-0 lg:border-r-0"
	>
		<div
			class="lg:w-1/3 w-full flex justify-center items-start flex-col border-b lg:border-r self-stretch p-8 border-white"
		>
			<a href="mailto:info@fueldao.io">info@fueldao.io</a>
		</div>
		<div
			class="lg:w-1/3 w-full flex justify-center items-start self-stretch p-8 flex-col lg:border-r border-white"
		>
			<div>Mumbai, India</div>
		</div>
		<div class="lg:w-1/3 lg:flex hidden justify-center items-start self-stretch p-8 flex-col"></div>
	</div>

	<form  class="flex flex-col lg:gap-4 gap-2">
		<div class="text-xl lg:text-5xl font-thin">Stay informed, join our newsletter</div>
			<div class="lg:text-xl font-thin pt-2">Enter your email here</div>
			<div class="flex items-center gap-2 lg:gap-4">
				<input
					required
					type="email"
					placeholder="youremail@email.com"
					class="lg:text-2xl text-lg border border-r-0 border-t-0 border-white/50 focus:border-white transition-colors lg:p-2 p-1 border-l-0 bg-transparent focus:outline-none placeholder:text-white/30"
				/>
				// <Button   class="lg:text-lg lg:py-3 lg:px-8 py-2 px-6">Submit</Button>
			</div>
	</form>
    <div class="h-20 w-full lg:h-1"></div>
</footer>
    }
}