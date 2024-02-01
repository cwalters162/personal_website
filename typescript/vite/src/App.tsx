import MenuBar from "./components/MenuBar.tsx"
import BodyContent from "./components/BodyContent.tsx"
import Footer from "./components/Footer.tsx"
import HeroSection from "./components/BodyContent/HeroSection.tsx"

function App() {
  return (
    <div
      className={
        "flex min-h-screen flex-col justify-between bg-light dark:bg-dark dark:text-light"
      }
    >
      <MenuBar />
      <HeroSection />
      <BodyContent />
      <Footer />
    </div>
  )
}

export default App
