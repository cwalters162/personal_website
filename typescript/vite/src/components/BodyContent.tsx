import Card from "./Card.tsx"
import HeroSection from "./BodyContent/HeroSection.tsx"

export default function BodyContent() {
  return (
    <div>
      <HeroSection />
      <div className={"flex flex-col md:flex-row"}>
        <Card
          title={"Team Player"}
          body={
            "I thrive in cross-functional team environments where I find best use of my ability to collaborate effectively."
          }
        />
        <Card
          title={"Experience"}
          body={
            "I have built robust and scalable cloud native web applications. Deployed on cloud platforms, using best practices in CI/CD in support of solve complex business challenges."
          }
        />
        <Card
          title={"Learning"}
          body={
            "I highly value continued learning and always strive to be the best in what I do. Enhancing my skills and expanding my knowledge base isa constant endeavor."
          }
        />
      </div>
      <div>
        <div>
          <h1>Projects</h1>
          <div>Game Of Life</div>
          <div>Breeding Colors</div>
          <div>Tic-Tac-Toe</div>
        </div>
      </div>
    </div>
  )
}