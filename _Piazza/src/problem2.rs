pub struct MatrixPosition {
    pub rowIndex: u32,
    pub colIndex: u32,
    pub val: u32,
}

pub fn createMatix(elements: Vec<MatrixPosition>) -> Option<Matrix> {
    let rowsCount = _getDeclaredRowsCount(&elements);
    let colsCount = _getDeclaredColsCount(&elements);

    let areCandidtesFormingMatrix = _areCandidatesFormingMatrix(rowsCount, colsCount, &elements);

    if !areCandidtesFormingMatrix {
        return None;
    } else {
        let matrix = Matrix::fromElements(elements);
        Option::from(matrix)
    }
}

fn _getDeclaredRowsCount(elements: &Vec<MatrixPosition>) -> u32 {
    let rowIndexes = elements
        .clone()
        .sort_by_key(|x| x.rowIndex);
    
    let maxIndex = rowIndexes.last();

    maxIndex + 1
}

fn _getDeclaredColsCount(elements: &Vec<MatrixPosition>) -> u32 {
    let colIdexes = elements
        .clone()
        .sort_by_key(|x| x.colIndex);
    
    let maxIndex = colIdexes.last();

    maxIndex + 1
}

fn _areCandidatesFormingMatrix(rowsCount: u32, colsCount: u32, elements: &Vec<MatrixPosition>) -> bool {

    true
}





pub struct Matrix {
    _elements: Vec<MatrixPosition>,
}


impl Matrix {
    fn fromElements(elements: Vec<MatrixPosition>) -> Matrix {
        let matrix = Matrix {
            _elements: elements,
        };
        matrix
    }

    fn getElem(&self, rowIndex: u32, colIndex: u32) -> MatrixPosition
    {
        MatrixPosition {roIndex: 0, colIndex: 0, val: 0}
    }

    pub fn rowsCount() -> u32 {
        2
    }

    pub fn colsCount() -> u32 {
        2
    }

    pub fn multiply(a: &Matrix, b: &Matrix) -> Matrix {}
}
