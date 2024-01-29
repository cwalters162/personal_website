interface CardProps {
  title: string
  body: string
}

export default function Card({ title, body }: CardProps) {
  return (
    <div className={"flex-1 p-4 text-center"}>
      <h1 className={"dark:text-primary-dark text-3xl"}>{title}</h1>
      <p>{body}</p>
    </div>
  )
}
