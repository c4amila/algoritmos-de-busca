use std::time::Instant;

fn main() {
    println!("tamanho | simples | interrupcao | contagem | tempo_ocorrencias | total_contagem | total_posicoes");
    let tamanhos = [1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000];

    for tamanho in tamanhos{
        experimento_buscas_multiplas(tamanho, 1000);
    }

}

//gera as strings
fn gerar_vetor(tamanho: usize) -> Vec<String> {
    let mut vetor = Vec::new();

    for i in 0..tamanho {
        vetor.push(format!("nome{}", i));
    }

    vetor
}

// busca Simples
fn busca_sequencial_simples(vetor: &Vec<String>, valor: &str) -> Option<usize> {
    for i in 0..vetor.len() {
        if vetor[i] == valor {
            return Some(i);
        }
    }
    None
}

//busca com interrupção 
fn busca_sequencial_interrompida(vetor: &Vec<String>, valor: &str) -> Option<usize> {
    for i in 0..vetor.len() {
        if vetor[i] == valor {
            return Some(i);
        }
        if vetor[i] == "nome9999" {
            break; 
        }
    }
    None
}

fn conta_elementos(vetor: &Vec<String>, valor:&str) -> usize {
    let mut count = 0;
    for elemento in vetor {
        if elemento == valor {
            count += 1;
        }
    }
    count
}

fn encontrar_posicao(vetor: &Vec<String>, valor: &str) -> Vec<usize> {
    let mut posicao = Vec::new();

    for i in 0..vetor.len() {
            if vetor[i] == valor {
                posicao.push(i);
            }
        }
    posicao
}

fn experimento_buscas_multiplas(tamanho: usize, num_buscas: usize) {
 let vetor = gerar_vetor(tamanho);
 
 // Busca Simples
 let inicio = Instant::now();
 for _ in 0..num_buscas {
 let _ = busca_sequencial_simples(&vetor, "nome9999");
 }
 let tempo_simples = inicio.elapsed();
 
 // Busca com Interrupção
 let inicio = Instant::now();
 for _ in 0..num_buscas {
 let _ = busca_sequencial_interrompida(&vetor, "nome9999");
 }
 let tempo_interrupcao = inicio.elapsed();

 //conta os elementos
 let inicio = Instant::now();
 let mut total = 0;
 for _ in 0..num_buscas {
     total += conta_elementos(&vetor, "nome9999");
 }
 let tempo_contagem = inicio.elapsed();

 //pega as posições
 let inicio = Instant::now();
 let mut total_posicoes = 0;
 for _ in 0..num_buscas {
     let resultado = encontrar_posicao(&vetor, "nome9999");
     total_posicoes += resultado.len();
 }
 let tempo_ocorrencias = inicio.elapsed();
 
/* println!("\n{} buscas com strings em vetor de tamanho {}:", num_buscas, tamanho);
 println!(" Simples: {:?}", tempo_simples);
 println!(" Interrupção: {:?}", tempo_interrupcao);
 println!(" Contagem: {:?}", tempo_contagem);
 println!(" Total de elementos encontrados: {}", total);
 println!(" Tempo para encontrar posições: {:?}", tempo_ocorrencias);
 println!(" Total de posições encontradas: {}", total_posicoes);*/
 
 println!("-----------------------------------------------------------");
 println!("{}, {:?}, {:?}, {:?}, {:?}, {}, {}", 
 tamanho, tempo_simples, tempo_interrupcao, tempo_contagem,tempo_ocorrencias, total, total_posicoes);
}

