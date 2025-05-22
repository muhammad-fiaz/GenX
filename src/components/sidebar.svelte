<aside
	class="flex flex-col items-center text-white shadow bg-[#1b1b1b] overflow-y-auto"
	style="height: calc(100vh - 2rem);"
>
	<!-- Logo -->
	<div class="h-16 flex items-center w-full">
		<a href="/" class="mx-auto h-6 w-6">
			<img
				src="/logo-rounded.png"
				alt="logo"
				class="h-6 w-6"
			/>
		</a>
	</div>


	<!-- Nav -->
	<ul class="flex-grow w-full overflow-y-auto overflow-x-hidden space-y-1 py-2">
		<li>
			<a
				href="."
				aria-label="Home"
				class="h-12 w-full flex justify-center items-center px-6 hover:bg-gray-500 focus:text-orange-500"
			>
				<svg
					class="h-5 w-5"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					viewBox="0 0 24 24"
				>
					<polyline points="22 12 16 12 14 15 10 15 8 12 2 12" />
					<path
						d="M5.45 5.11L2 12v6a2 2 0 002 2h16a2 2 0 002-2v-6l-3.45-6.89A2 2 0 0016.76 4H7.24a2 2 0 00-1.79 1.11z"
					/>
				</svg>
			</a>
		</li>
		<li>
			<a
				href="."
				aria-label="Bookmarks"
				class="h-12 w-full flex justify-center items-center px-6 hover:bg-gray-500 focus:text-orange-500"
			>
				<svg
					class="h-5 w-5"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					viewBox="0 0 24 24"
				>
					<path d="M19 21l-7-5-7 5V5a2 2 0 012-2h10a2 2 0 012 2z" />
				</svg>
			</a>
		</li>
	</ul>

	<!-- Footer -->
	<div class="h-16 flex items-center w-full">
		<button
			aria-label="Logout"
			class="h-16 w-full flex justify-center items-center hover:bg-red-200 focus:outline-none focus:text-orange-500"
		>
			<svg
				class="h-5 w-5 text-red-700"
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				viewBox="0 0 24 24"
			>
				<path d="M9 21H5a2 2 0 01-2-2V5a2 2 0 012-2h4" />
				<polyline points="16 17 21 12 16 7" />
				<line x1="21" y1="12" x2="9" y2="12" />
			</svg>
		</button>
	</div>
</aside>