
import { AppShell } from "./components/layout/AppShell"
import  UserSetup from "./components/user/userSetup"

export default function App() {
	return (

		<UserSetup>
			{(onEditUser) => (
				<AppShell onEditUser={onEditUser} />
			)}
		</UserSetup>

	)
}
