enum Direction:
  case Up, Down, Left, Right
  def turnRight: Direction = this match
    case Up => Right
    case Right => Down
    case Down => Left
    case Left => Up
  
  def getOffset: (Int, Int) = this match
    case Up => (0, -1)
    case Down => (0, 1)
    case Left => (-1, 0)
    case Right => (1, 0)

@main def day06(): Unit =
  val source = io.Source.fromFile("input.txt")
  val lines = try source.mkString finally source.close()
  println(s"Part 1: ${part1(lines)}")
  println(s"Part 2: ${part2(lines)}")

def part1(input: String): Int = {
  val lines = input.split("\n")
  val obstacles = lines.zipWithIndex.flatMap { (line, y) =>
    line.zipWithIndex.collect { case ('#', x) => (x, y) }
  }.toSet
  val height = lines.length
  val width = lines(0).length

  val visitedImmutable: Array[((Int, Int), collection.mutable.Set[Direction])] = lines.zipWithIndex.flatMap { (line, y) => {
    line.zipWithIndex.collect { case ('^', x) => ((x, y), collection.mutable.Set(Direction.Up)) }
  }}
  var visited = collection.mutable.Map(visitedImmutable*)
  val ((startX, startY), _) = visited.head

  var currentOffset = Direction.Up.getOffset
  var currentX = startX + currentOffset._1
  var currentY = startY + currentOffset._2
  var currentDirection = Direction.Up
  while (!visited.getOrElse((currentX, currentY), collection.mutable.Set()).contains(currentDirection) && currentX >= 0 && currentX < width && currentY >= 0 && currentY < height) {
    if (obstacles.contains((currentX, currentY))) {
      currentX -= currentOffset._1
      currentY -= currentOffset._2
      currentDirection = currentDirection.turnRight
      currentOffset = currentDirection.getOffset
      currentX += currentOffset._1
      currentY += currentOffset._2
    } else {
      if (!visited.contains((currentX, currentY))) {
        visited((currentX, currentY)) = collection.mutable.Set()
      }
      visited((currentX, currentY)) += currentDirection
      currentX += currentOffset._1
      currentY += currentOffset._2
    }
  }

  visited.size
}

def part2(input: String): Int = {
  val lines = input.split("\n")
  val obstacles = lines.zipWithIndex.flatMap { (line, y) =>
    line.zipWithIndex.collect { case ('#', x) => (x, y) }
  }.toSet
  val height = lines.length
  val width = lines(0).length

  val (startX, startY): (Int, Int) = lines.zipWithIndex.flatMap { (line, y) => {
    line.zipWithIndex.collect { case ('^', x) => (x, y) }
  }}.head

  var count = 0

  (0 to height - 1) foreach { y =>
    (0 to width - 1) foreach { x =>
      if (!obstacles.contains((x, y))) {
        if (checkLoop((obstacles + ((x, y))), startX, startY, Direction.Up, height, width)) {
          count += 1
        }
      }
    }
  }

  count
}

def checkLoop(obstacles: Set[(Int, Int)], startX: Int, startY: Int, startDirection: Direction, height: Int, width: Int): Boolean = {
  var visited = collection.mutable.Map((startX, startY) -> collection.mutable.Set(startDirection))

  var currentOffset = startDirection.getOffset
  var currentX = startX + currentOffset._1
  var currentY = startY + currentOffset._2
  var currentDirection = startDirection
  while (!visited.getOrElse((currentX, currentY), collection.mutable.Set()).contains(currentDirection)) {
    if (currentX < 0 || currentX >= width || currentY < 0 || currentY >= height) {
      return false
    }
    if (obstacles.contains((currentX, currentY))) {
      currentX -= currentOffset._1
      currentY -= currentOffset._2
      currentDirection = currentDirection.turnRight
      currentOffset = currentDirection.getOffset
      currentX += currentOffset._1
      currentY += currentOffset._2
    } else {
      if (!visited.contains((currentX, currentY))) {
        visited((currentX, currentY)) = collection.mutable.Set()
      }
      visited((currentX, currentY)) += currentDirection
      currentX += currentOffset._1
      currentY += currentOffset._2
    }
  }
  return true
}
