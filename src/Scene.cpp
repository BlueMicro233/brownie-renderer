//
// Created by Göksu Güvendiren on 2019-05-14.
//

#include "Scene.hpp"
#include "Material.hpp"


void Scene::buildBVH() {
    printf(" - Generating BVH...\n\n");
    this->bvh = new BVHAccel(objects, 1, BVHAccel::SplitMethod::NAIVE);
}

Intersection Scene::intersect(const Ray &ray) const
{
    return this->bvh->Intersect(ray);
}

void Scene::sampleLight(Intersection &pos, float &pdf) const
{
    float emit_area_sum = 0;
    for (uint32_t k = 0; k < objects.size(); ++k) {
        if (objects[k]->hasEmit()){
            emit_area_sum += objects[k]->getArea();
        }
    }
    float p = get_random_float() * emit_area_sum;
    emit_area_sum = 0;
    for (uint32_t k = 0; k < objects.size(); ++k) {
        if (objects[k]->hasEmit()){
            emit_area_sum += objects[k]->getArea();
            if (p <= emit_area_sum){
                objects[k]->Sample(pos, pdf);
                break;
            }
        }
    }
}

bool Scene::trace(
        const Ray &ray,
        const std::vector<Object*> &objects,
        float &tNear, uint32_t &index, Object **hitObject)
{
    *hitObject = nullptr;
    for (uint32_t k = 0; k < objects.size(); ++k) {
        float tNearK = kInfinity;
        uint32_t indexK;
        Vector2f uvK;
        if (objects[k]->intersect(ray, tNearK, indexK) && tNearK < tNear) {
            *hitObject = objects[k];
            tNear = tNearK;
            index = indexK;
        }
    }


    return (*hitObject != nullptr);
}


// Implementation of Path Tracing
Vector3f Scene::castRay(const Ray &ray, int depth) const
{
    // TODO Implement Path Tracing Algorithm here
    // shade(p, wo)
    //  sampleLight(inter , pdf_light)
    //  Get x, ws, NN, emit from inter
    //  Shoot a ray from p to x
    //  If the ray is not blocked in the middle
    //  L_dir = emit * eval(wo, ws, N) * dot(ws, N) * dot(ws, NN) / |x-p|^2 / pdf_light
    Vector3f L_dir = 0.f;

    // initial hit point
    Intersection p = intersect(ray);

    if (!p.happened)
    {
        return Vector3f(0.f, 0.f, 0.f);
    }

    // If we directly shoot the light area, we return the emit energy.
    // This is L_e term in the full rendering equation.
    if (p.m->hasEmission())
    {
        if (depth == 0)
        {
            return p.m->getEmission();
        }
        else
        {
            return Vector3f(0.f, 0.f, 0.f);
        }
    }

    // outgoing direction
    Vector3f wo = (ray.origin - p.coords).norm();

    /*
     * Direct Lighting
     * We do light sampling here.
     */

    // x is sample point on scene area light
    Intersection x;
    float pdf_arealight = 0.f;
    sampleLight(x, pdf_arealight);

    // Direction between hit point and light samples.
    Vector3f ws = normalize(x.coords - p.coords);

    // We trace shadow ray to determine the direct lighting.
    Ray shadowRay(p.coords + ws * 0.0005f, ws);
    Intersection isBlock = intersect(shadowRay);

    // Check if shadow ray's hit point is light sample x
    // If there's occlusion cause by other object, the p is in shadow (related to x), so L_dir = 0.
    if (isBlock.happened && (isBlock.coords - x.coords).norm() < 0.001f)
    {
        L_dir = x.emit
                * p.m->eval(wo, ws, p.normal)
                * dotProduct(ws, p.normal)
                * dotProduct(-ws, x.normal)
                / ((x.coords - p.coords).norm() * (x.coords - p.coords).norm())
                / pdf_arealight;
    }
    else
    {
        L_dir = 0.f;
    }

    /*
     * Indirect Lighting
     * multi-bounce indirect illumination
     */

    //  L_indir = 0.0
    //  Test Russian Roulette with probability RussianRoulette
    //  wi = sample(wo, N)
    //  Trace a ray r(p, wi)
    //  If ray r hit a non -emitting object at q
    //  L_indir = shade(q, wi) * eval(wo, wi, N) * dot(wi, N) / pdf(wo, wi, N) / RussianRoulette
    //  Return L_dir + L_indir
    Vector3f L_indir = 0.f;

    // min bounces: 4
    if (depth > 4)
    {
        if (get_random_float() > RussianRoulette)
        {
            return L_dir;
        }
    }

    Vector3f wi = p.m->sample(wo, p.normal);
    float cos_theta = dotProduct(wi, p.normal);
    float pdf_indir = cos_theta / M_PI;

    // Ensure that cos_theta > 0 (since we are in a Hemispherical Integral Domain) and pdf > 0.
    if (pdf_indir > 0.f && cos_theta > 0.f)
    {
        Vector3f shading = castRay(Ray(p.coords + wi * 0.0005f, wi), depth + 1);
        // Intersection q = intersect(Ray(p.coords, wi));

        // Indirect illumination should not add emit itself directly.
        // We trace 4-bounce ray for every pixel, when it comes to >4, we use RR to terminate.
        float rr = (depth > 4) ? RussianRoulette : 1.f;

        L_indir = shading * p.m->eval(wo, wi, p.normal) * cos_theta
                / pdf_indir
                / rr;
    }

    return L_dir + L_indir;
}