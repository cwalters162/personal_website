import MenuBar from "./components/MenuBar.tsx"
import BodyContent from "./components/BodyContent.tsx"

function App() {
  return (
    <div className={"bg-light dark:bg-dark dark:text-light"}>
      <MenuBar />
      <BodyContent />
      <div className={"h-[2000px] bg-green-400"}>this is a thing</div>
    </div>
  )
}

export default App
