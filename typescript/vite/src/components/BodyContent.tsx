import Card from "./Card.tsx"
import Portrait from "../assets/create-verity-logo-2.png"

export default function BodyContent() {
  return (
    <div>
      <div>
        <div className={"flex"}>
          <div className={"pl-10"}>
            <h1 className={"pb-3 text-5xl"}>
              <span className={"dark:text-primary-dark"}>Best software </span>
              <span>you have ever seen</span>
            </h1>
            <div className={"flex justify-center pt-10"}>
              <p className={"w-2/3 text-center text-3xl"}>
                <span>Experienced </span>
                software engineer using
                <span className={"dark:text-primary-dark"}>
                  {" "}
                  best practices{" "}
                </span>
                and
                <span className={"dark:text-primary-dark"}> scalable </span>
                technologies to
                <span className={"dark:text-primary-dark"}> exceed </span>
                your needs
              </p>
            </div>
            <div className={"pt-10 text-center"}>
              <button className={"bg-primary-dark rounded-full p-2"}>
                Contact Now
              </button>
            </div>
          </div>
          <div
            className={
              "justify-middle flex hidden overflow-clip rounded-full bg-red-400 sm:block"
            }
          >
            <img src={Portrait} alt={"headshot of Cherokee Walters"} />
          </div>
        </div>
      </div>
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
